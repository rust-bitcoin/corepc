#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, cfg_attr(all(), doc = include_str!("../README.md")))]

pub extern crate corepc_client as client;

#[rustfmt::skip]
mod client_versions;
mod versions;

use std::ffi::OsStr;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::Duration;
use std::{env, fmt, fs, thread};

use anyhow::Context;
use corepc_client::client_sync::{self, Auth};
use tempfile::TempDir;
pub use {anyhow, serde_json, tempfile, which};

#[rustfmt::skip]                // Keep pubic re-exports separate.
#[doc(inline)]
pub use self::{
    // Re-export `vtype` (the version specific types) and client defined types.
    client_versions::*,
    // Re-export the version string e.g., "28.0".
    versions::VERSION,
    // Re-export the model types as `mtype` to differentiate it from `vtype`.
    client::types::model as mtype, // `types` is the `corepc-types` crate.
};

#[derive(Debug)]
/// Struct representing the bitcoind process with related information
pub struct Node {
    /// Process child handle, used to terminate the process when this struct is dropped
    process: Child,
    /// Rpc client linked to this bitcoind process
    pub client: Client,
    /// Work directory, where the node store blocks and other stuff.
    work_dir: DataDir,

    /// Contains information to connect to this node
    pub params: ConnectParams,
}

#[derive(Debug)]
/// The DataDir struct defining the kind of data directory the node
/// will contain. Data directory can be either persistent, or temporary.
pub enum DataDir {
    /// Persistent Data Directory
    Persistent(PathBuf),
    /// Temporary Data Directory
    Temporary(TempDir),
}

impl DataDir {
    /// Return the data directory path
    fn path(&self) -> PathBuf {
        match self {
            Self::Persistent(path) => path.to_owned(),
            Self::Temporary(tmp_dir) => tmp_dir.path().to_path_buf(),
        }
    }
}

#[derive(Debug, Clone)]
/// Contains all the information to connect to this node
pub struct ConnectParams {
    /// Path to the node cookie file, useful for other client to connect to the node
    pub cookie_file: PathBuf,
    /// Url of the rpc of the node, useful for other client to connect to the node
    pub rpc_socket: SocketAddrV4,
    /// p2p connection url, is some if the node started with p2p enabled
    pub p2p_socket: Option<SocketAddrV4>,
    /// zmq pub raw block connection url
    pub zmq_pub_raw_block_socket: Option<SocketAddrV4>,
    /// zmq pub raw tx connection Url
    pub zmq_pub_raw_tx_socket: Option<SocketAddrV4>,
}

pub struct CookieValues {
    pub user: String,
    pub password: String,
}

impl ConnectParams {
    /// Parses the cookie file content
    fn parse_cookie(content: String) -> Option<CookieValues> {
        let values: Vec<_> = content.splitn(2, ':').collect();
        let user = values.first()?.to_string();
        let password = values.get(1)?.to_string();
        Some(CookieValues { user, password })
    }

    /// Return the user and password values from cookie file
    pub fn get_cookie_values(&self) -> Result<Option<CookieValues>, std::io::Error> {
        let cookie = std::fs::read_to_string(&self.cookie_file)?;
        Ok(self::ConnectParams::parse_cookie(cookie))
    }
}

/// Enum to specify p2p settings
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum P2P {
    /// the node doesn't open a p2p port and work in standalone mode
    No,
    /// the node open a p2p port
    Yes,
    /// The node open a p2p port and also connects to the url given as parameter, it's handy to
    /// initialize this with [Node::p2p_connect] of another node. The `bool` parameter indicates
    /// if the node can accept connection too.
    Connect(SocketAddrV4, bool),
}

/// All the possible error in this crate
pub enum Error {
    /// Wrapper of io Error
    Io(std::io::Error),
    /// Wrapper of bitcoincore_rpc Error
    Rpc(client_sync::Error),
    /// Returned when calling methods requiring a feature to be activated, but it's not
    NoFeature,
    /// Returned when calling methods requiring a env var to exist, but it's not
    NoEnvVar,
    /// Returned when calling methods requiring the bitcoind executable but none is found
    /// (no feature, no `BITCOIND_EXE`, no `bitcoind` in `PATH` )
    NoBitcoindExecutableFound,
    /// Wrapper of early exit status
    EarlyExit(ExitStatus),
    /// Returned when both tmpdir and staticdir is specified in `Conf` options
    BothDirsSpecified,
    /// Returned when -rpcuser and/or -rpcpassword is used in `Conf` args
    /// It will soon be deprecated, please use -rpcauth instead
    RpcUserAndPasswordUsed,
    /// Returned when expecting an auto-downloaded executable but `BITCOIND_SKIP_DOWNLOAD` env var is set
    SkipDownload,
    /// Returned when bitcoind could not be reached after multiple attempts.
    /// The attached string, if present, contains the error encountered when trying to connect.
    NoBitcoindInstance(String),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            Io(_) => write!(f, "io::Error"), // FIXME: Use bitcoin-internals.
            Rpc(_) => write!(f, "bitcoin_rpc::Error"),
            NoFeature => write!(f, "Called a method requiring a feature to be set, but it's not"),
            NoEnvVar => write!(f, "Called a method requiring env var `BITCOIND_EXE` to be set, but it's not"),
            NoBitcoindExecutableFound =>  write!(f, "`bitcoind` executable is required, provide it with one of the following: set env var `BITCOIND_EXE` or use a feature like \"22_1\" or have `bitcoind` executable in the `PATH`"),
            EarlyExit(e) => write!(f, "The bitcoind process terminated early with exit code {}", e),
            BothDirsSpecified => write!(f, "tempdir and staticdir cannot be enabled at same time in configuration options"),
            RpcUserAndPasswordUsed => write!(f, "`-rpcuser` and `-rpcpassword` cannot be used, it will be deprecated soon and it's recommended to use `-rpcauth` instead which works alongside with the default cookie authentication"),
            SkipDownload => write!(f, "expecting an auto-downloaded executable but `BITCOIND_SKIP_DOWNLOAD` env var is set"),
            NoBitcoindInstance(msg) => write!(f, "it appears that bitcoind is not reachable: {}", msg),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self) }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use Error::*;

        match *self {
            Error::Io(ref e) => Some(e),
            Error::Rpc(ref e) => Some(e),
            NoFeature
            | NoEnvVar
            | NoBitcoindExecutableFound
            | EarlyExit(_)
            | BothDirsSpecified
            | RpcUserAndPasswordUsed
            | SkipDownload
            | NoBitcoindInstance(_) => None,
        }
    }
}

const LOCAL_IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

const INVALID_ARGS: [&str; 2] = ["-rpcuser", "-rpcpassword"];

/// The node configuration parameters, implements a convenient [Default] for most common use.
///
/// `#[non_exhaustive]` allows adding new parameters without breaking downstream users.
/// Users cannot instantiate the struct directly, they need to create it via the `default()` method
/// and mutate fields according to their preference.
///
/// Default values:
/// ```
/// use corepc_node as bitcoind;
/// let mut conf = bitcoind::Conf::default();
/// conf.args = vec!["-regtest", "-fallbackfee=0.0001"];
/// conf.view_stdout = false;
/// conf.p2p = bitcoind::P2P::No;
/// conf.network = "regtest";
/// conf.tmpdir = None;
/// conf.staticdir = None;
/// conf.attempts = 5;
/// assert_eq!(conf, bitcoind::Conf::default());
/// ```
///
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Conf<'a> {
    /// Bitcoind command line arguments containing no spaces like `vec!["-dbcache=300", "-regtest"]`
    /// note that `port`, `rpcport`, `connect`, `datadir`, `listen`
    /// cannot be used because they are automatically initialized.
    pub args: Vec<&'a str>,

    /// if `true` bitcoind log output will not be suppressed
    pub view_stdout: bool,

    /// Allows to specify options to open p2p port or connect to the another node
    pub p2p: P2P,

    /// Must match what specified in args without dashes, needed to locate the cookie file
    /// directory with different/esoteric networks
    pub network: &'a str,

    /// Temporary directory path.
    ///
    /// Optionally specify a temporary or persistent working directory for the node.
    /// The following two parameters can be configured to simulate desired working directory configuration.
    ///
    /// tmpdir is Some() && staticdir is Some() : Error. Cannot be enabled at same time.
    /// tmpdir is Some(temp_path) && staticdir is None : Create temporary directory at `tmpdir` path.
    /// tmpdir is None && staticdir is Some(work_path) : Create persistent directory at `staticdir` path.
    /// tmpdir is None && staticdir is None: Creates a temporary directory in OS default temporary directory (eg /tmp) or `TEMPDIR_ROOT` env variable path.
    ///
    /// It may be useful for example to set to a ramdisk via `TEMPDIR_ROOT` env option so that
    /// bitcoin nodes spawn very fast because their datadirs are in RAM. Should not be enabled with persistent
    /// mode, as it cause memory overflows.
    pub tmpdir: Option<PathBuf>,

    /// Persistent directory path
    pub staticdir: Option<PathBuf>,

    /// Try to spawn the process `attempt` time
    ///
    /// The OS is giving available ports to use, however, they aren't booked, so it could rarely
    /// happen they are used at the time the process is spawn. When retrying other available ports
    /// are returned reducing the probability of conflicts to negligible.
    pub attempts: u8,

    /// Enable the ZMQ interface to be accessible.
    pub enable_zmq: bool,

    /// Load `wallet` after initialization.
    pub wallet: Option<String>,
}

impl Default for Conf<'_> {
    fn default() -> Self {
        Conf {
            args: vec!["-regtest", "-fallbackfee=0.0001"],
            view_stdout: false,
            p2p: P2P::No,
            network: "regtest",
            tmpdir: None,
            staticdir: None,
            attempts: 5,
            enable_zmq: false,
            wallet: Some("default".to_string()),
        }
    }
}

impl Node {
    /// Launch the bitcoind process from the given `exe` executable with default args.
    ///
    /// Waits for the node to be ready to accept connections before returning
    pub fn new<S: AsRef<OsStr>>(exe: S) -> anyhow::Result<Node> {
        Node::with_conf(exe, &Conf::default())
    }

    /// Launch the bitcoind process from the given `exe` executable with given [Conf] param and
    /// create/load the "default" wallet.
    ///
    /// Waits for the node to be ready to accept connections before returning.
    ///
    /// # Parameters
    ///
    /// * `exe` - The path to the bitcoind executable.
    /// * `conf` - The configuration parameters for the node.
    ///
    /// # Returns
    ///
    /// A [`Node`] instance if the node is successfully started and ready to accept connections.
    ///
    /// # Errors
    ///
    /// If the node fails to start after the specified number of attempts.
    pub fn with_conf<S: AsRef<OsStr>>(exe: S, conf: &Conf) -> anyhow::Result<Node> {
        for attempt in 0..conf.attempts {
            let work_dir = Self::init_work_dir(conf)?;
            let cookie_file = work_dir.path().join(conf.network).join(".cookie");

            let rpc_port = get_available_port()?;
            let rpc_socket = SocketAddrV4::new(LOCAL_IP, rpc_port);
            let rpc_url = format!("http://{}", rpc_socket);

            let (p2p_args, p2p_socket) = Self::p2p_args(&conf.p2p)?;
            let (zmq_args, zmq_pub_raw_tx_socket, zmq_pub_raw_block_socket) =
                Self::zmq_args(conf.enable_zmq)?;

            let stdout = if conf.view_stdout { Stdio::inherit() } else { Stdio::null() };

            let datadir_arg = format!("-datadir={}", work_dir.path().display());
            let rpc_arg = format!("-rpcport={}", rpc_port);
            let default_args = [&datadir_arg, &rpc_arg];
            let conf_args = validate_args(conf.args.clone())?;

            let mut process = Command::new(exe.as_ref())
                .args(default_args)
                .args(&p2p_args)
                .args(&conf_args)
                .args(&zmq_args)
                .stdout(stdout)
                .spawn()
                .with_context(|| format!("Error while executing {:?}", exe.as_ref()))?;
            match process.try_wait() {
                Ok(Some(_)) | Err(_) => {
                    // Process has exited or an error occurred, kill and retry
                    let _ = process.kill();
                    continue;
                }
                Ok(None) => {
                    // Process is still running, proceed
                }
            }

            if Self::wait_for_cookie_file(cookie_file.as_path(), Duration::from_secs(5)).is_err() {
                // If the cookie file is not accessible a new work_dir is needed and therefore a new
                // process. Kill the process and retry.
                let _ = process.kill();
                continue;
            }
            let auth = Auth::CookieFile(cookie_file.clone());

            let client_base = Self::create_client_base(&rpc_url, &auth)?;
            let client = match &conf.wallet {
                Some(wallet) => {
                    match Self::create_client_wallet(&client_base, &rpc_url, &auth, wallet) {
                        Ok(client) => client,
                        Err(e) => {
                            if attempt == conf.attempts - 1 {
                                return Err(e);
                            } else {
                                // If the wallet cannot be created or loaded, there might be an issue
                                // with the work_dir or process. Kill the process and retry.
                                let _ = process.kill();
                                continue;
                            }
                        }
                    }
                }
                None => client_base,
            };
            if Self::wait_for_client(&client, Duration::from_secs(5)).is_err() {
                // If the client times out there might be an issue with the work_dir or process. Kill
                // the process and retry.
                let _ = process.kill();
                continue;
            }

            return Ok(Node {
                process,
                client,
                work_dir,
                params: ConnectParams {
                    cookie_file,
                    rpc_socket,
                    p2p_socket,
                    zmq_pub_raw_block_socket,
                    zmq_pub_raw_tx_socket,
                },
            });
        }
        Err(anyhow::anyhow!("Failed to start the node after {} attempts", conf.attempts))
    }

    /// Initialize the work directory based on the provided configuration in [`Conf`].
    ///
    /// # Parameters
    ///
    /// * `conf` - Contains the paths for temporary (`tmpdir`) and static (`staticdir`)
    ///   directories. If neither is specified, a temporary directory will be created in the
    ///   system's default temporary directory.
    fn init_work_dir(conf: &Conf) -> anyhow::Result<DataDir> {
        let tmpdir =
            conf.tmpdir.clone().or_else(|| env::var("TEMPDIR_ROOT").map(PathBuf::from).ok());
        let work_dir = match (&tmpdir, &conf.staticdir) {
            (Some(_), Some(_)) => return Err(Error::BothDirsSpecified.into()),
            (Some(tmpdir), None) => DataDir::Temporary(TempDir::new_in(tmpdir)?),
            (None, Some(workdir)) => {
                fs::create_dir_all(workdir)?;
                DataDir::Persistent(workdir.to_owned())
            }
            (None, None) => DataDir::Temporary(TempDir::new()?),
        };
        Ok(work_dir)
    }

    /// Returns the p2p args and the p2p socket address if any.
    fn p2p_args(p2p: &P2P) -> anyhow::Result<(Vec<String>, Option<SocketAddrV4>)> {
        match p2p {
            P2P::No => Ok((vec!["-listen=0".to_string()], None)),
            P2P::Yes => {
                let p2p_port = get_available_port()?;
                let p2p_socket = SocketAddrV4::new(LOCAL_IP, p2p_port);
                let bind_arg = format!("-bind={}", p2p_socket);
                let args = vec![bind_arg];
                Ok((args, Some(p2p_socket)))
            }
            P2P::Connect(other_node_url, listen) => {
                let p2p_port = get_available_port()?;
                let p2p_socket = SocketAddrV4::new(LOCAL_IP, p2p_port);
                let bind_arg = format!("-bind={}", p2p_socket);
                let connect = format!("-connect={}", other_node_url);
                let mut args = vec![bind_arg, connect];
                if *listen {
                    args.push("-listen=1".to_string())
                }
                Ok((args, Some(p2p_socket)))
            }
        }
    }

    /// Returns the zmq args and the zmq socket addresses if any.
    ///
    /// # Parameters
    /// * `enable_zmq` - If `true`, creates two ZMQ sockets:
    ///     - `zmq_pub_raw_tx_socket`: for raw transaction publishing
    ///     - `zmq_pub_raw_block_socket`: for raw block publishing
    fn zmq_args(
        enable_zmq: bool,
    ) -> anyhow::Result<(Vec<String>, Option<SocketAddrV4>, Option<SocketAddrV4>)> {
        if enable_zmq {
            let zmq_pub_raw_tx_port = get_available_port()?;
            let zmq_pub_raw_tx_socket = SocketAddrV4::new(LOCAL_IP, zmq_pub_raw_tx_port);
            let zmq_pub_raw_block_port = get_available_port()?;
            let zmq_pub_raw_block_socket = SocketAddrV4::new(LOCAL_IP, zmq_pub_raw_block_port);
            let zmqpubrawblock_arg =
                format!("-zmqpubrawblock=tcp://0.0.0.0:{}", zmq_pub_raw_block_port);
            let zmqpubrawtx_arg = format!("-zmqpubrawtx=tcp://0.0.0.0:{}", zmq_pub_raw_tx_port);
            Ok((
                vec![zmqpubrawtx_arg, zmqpubrawblock_arg],
                Some(zmq_pub_raw_tx_socket),
                Some(zmq_pub_raw_block_socket),
            ))
        } else {
            Ok((vec![], None, None))
        }
    }

    /// Returns `Ok` once the cookie file is accessible, or an error if it times out.
    fn wait_for_cookie_file(cookie_file: &Path, timeout: Duration) -> anyhow::Result<()> {
        let start = std::time::Instant::now();
        while start.elapsed() < timeout {
            if cookie_file.exists() {
                return Ok(());
            }
            thread::sleep(Duration::from_millis(200));
        }
        Err(anyhow::anyhow!("timeout waiting for cookie file: {}", cookie_file.display()))
    }

    /// Returns `Ok` once the client can successfully call, or an error if it times out.
    fn wait_for_client(client: &Client, timeout: Duration) -> anyhow::Result<()> {
        let start = std::time::Instant::now();
        while start.elapsed() < timeout {
            // Test calling GetBlockchainInfo. Use serde value to be resilient to upstream changes.
            if client.call::<serde_json::Value>("getblockchaininfo", &[]).is_ok() {
                return Ok(());
            }
            thread::sleep(Duration::from_millis(200));
        }
        Err(anyhow::anyhow!("timeout waiting for client to be ready"))
    }

    /// Create a new RPC client connected to the given `rpc_url` with the provided `auth`.
    ///
    /// The client may not be immediately available, so retry up to 10 times.
    fn create_client_base(rpc_url: &str, auth: &Auth) -> anyhow::Result<Client> {
        for _ in 0..10 {
            if let Ok(client) = Client::new_with_auth(rpc_url, auth.clone()) {
                return Ok(client);
            }
            thread::sleep(Duration::from_millis(200));
        }
        Client::new_with_auth(rpc_url, auth.clone())
            .map_err(|e| Error::NoBitcoindInstance(e.to_string()).into())
    }

    /// Create a new RPC client connected to the given `wallet`.
    ///
    /// If the wallet with the given name does not exist, it will create it.
    /// If the wallet already exists, it will load it.
    ///
    /// The client or wallet may not be immediately available, so retry up to 10 times.
    fn create_client_wallet(
        client_base: &Client,
        rpc_url: &str,
        auth: &Auth,
        wallet: &str,
    ) -> anyhow::Result<Client> {
        for _ in 0..10 {
            // Try to create the wallet, or if that fails it might already exist so try to load it.
            if client_base.create_wallet(wallet).is_ok() || client_base.load_wallet(wallet).is_ok()
            {
                let url = format!("{}/wallet/{}", rpc_url, wallet);
                return Client::new_with_auth(&url, auth.clone())
                    .map_err(|e| Error::NoBitcoindInstance(e.to_string()).into());
            }
            thread::sleep(Duration::from_millis(200));
        }
        Err(Error::NoBitcoindInstance("Could not create or load wallet".to_string()).into())
    }

    /// Returns the rpc URL including the schema eg. http://127.0.0.1:44842
    pub fn rpc_url(&self) -> String { format!("http://{}", self.params.rpc_socket) }

    /// Returns the rpc URL including the schema and the given `wallet_name`
    /// eg. http://127.0.0.1:44842/wallet/my_wallet
    pub fn rpc_url_with_wallet<T: AsRef<str>>(&self, wallet_name: T) -> String {
        format!("http://{}/wallet/{}", self.params.rpc_socket, wallet_name.as_ref())
    }

    /// Return the current workdir path of the running node
    pub fn workdir(&self) -> PathBuf { self.work_dir.path() }

    /// Returns the [P2P] enum to connect to this node p2p port
    pub fn p2p_connect(&self, listen: bool) -> Option<P2P> {
        self.params.p2p_socket.map(|s| P2P::Connect(s, listen))
    }

    /// Stop the node, waiting correct process termination
    pub fn stop(&mut self) -> anyhow::Result<ExitStatus> {
        self.client.stop()?;
        Ok(self.process.wait()?)
    }

    /// Create a new wallet in the running node, and return an RPC client connected to the just
    /// created wallet
    pub fn create_wallet<T: AsRef<str>>(&self, wallet: T) -> anyhow::Result<Client> {
        let _ = self.client.create_wallet(wallet.as_ref())?;
        Ok(Client::new_with_auth(
            &self.rpc_url_with_wallet(wallet),
            Auth::CookieFile(self.params.cookie_file.clone()),
        )?)
    }
}

#[cfg(feature = "download")]
impl Node {
    /// create Node struct with the downloaded executable.
    pub fn from_downloaded() -> anyhow::Result<Node> { Node::new(downloaded_exe_path()?) }

    /// create Node struct with the downloaded executable and given Conf.
    pub fn from_downloaded_with_conf(conf: &Conf) -> anyhow::Result<Node> {
        Node::with_conf(downloaded_exe_path()?, conf)
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        if let DataDir::Persistent(_) = self.work_dir {
            let _ = self.stop();
        }
        let _ = self.process.kill();
    }
}

/// Returns a non-used local port if available.
///
/// Note there is a race condition during the time the method check availability and the caller
pub fn get_available_port() -> anyhow::Result<u16> {
    // using 0 as port let the system assign a port available
    let t = TcpListener::bind(("127.0.0.1", 0))?; // 0 means the OS choose a free port
    Ok(t.local_addr().map(|s| s.port())?)
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self { Error::Io(e) }
}

impl From<client_sync::Error> for Error {
    fn from(e: client_sync::Error) -> Self { Error::Rpc(e) }
}

/// Provide the bitcoind executable path if a version feature has been specified
#[cfg(not(feature = "download"))]
pub fn downloaded_exe_path() -> anyhow::Result<String> { Err(Error::NoFeature.into()) }

/// Provide the bitcoind executable path if a version feature has been specified
#[cfg(feature = "download")]
pub fn downloaded_exe_path() -> anyhow::Result<String> {
    if std::env::var_os("BITCOIND_SKIP_DOWNLOAD").is_some() {
        return Err(Error::SkipDownload.into());
    }

    let mut path: PathBuf = env!("OUT_DIR").into();
    path.push("bitcoin");
    path.push(format!("bitcoin-{}", VERSION));
    path.push("bin");

    if cfg!(target_os = "windows") {
        path.push("bitcoind.exe");
    } else {
        path.push("bitcoind");
    }

    let path = format!("{}", path.display());
    Ok(path)
}

/// Returns the daemon `bitcoind` executable with the following precedence:
///
/// 1) If it's specified in the `BITCOIND_EXE` env var
/// 2) If there is no env var but the auto-download feature is enabled, returns the
///    path of the downloaded executable
/// 3) If neither of the precedent are available, the `bitcoind` executable is searched in the `PATH`
pub fn exe_path() -> anyhow::Result<String> {
    if let Ok(path) = std::env::var("BITCOIND_EXE") {
        return Ok(path);
    }
    if let Ok(path) = downloaded_exe_path() {
        return Ok(path);
    }
    which::which("bitcoind")
        .map_err(|_| Error::NoBitcoindExecutableFound.into())
        .map(|p| p.display().to_string())
}

/// Validate the specified arg if there is any unavailable or deprecated one
pub fn validate_args(args: Vec<&str>) -> anyhow::Result<Vec<&str>> {
    args.iter().try_for_each(|arg| {
        // other kind of invalid arguments can be added into the list if needed
        if INVALID_ARGS.iter().any(|x| arg.starts_with(x)) {
            return Err(Error::RpcUserAndPasswordUsed);
        }
        Ok(())
    })?;

    Ok(args)
}

#[cfg(test)]
mod test {
    use std::net::SocketAddrV4;

    use tempfile::TempDir;

    use super::*;
    use crate::{exe_path, get_available_port, Conf, Node, LOCAL_IP, P2P};

    #[test]
    fn test_local_ip() {
        assert_eq!("127.0.0.1", format!("{}", LOCAL_IP));
        let port = get_available_port().unwrap();
        let socket = SocketAddrV4::new(LOCAL_IP, port);
        assert_eq!(format!("127.0.0.1:{}", port), format!("{}", socket));
    }

    #[test]
    fn test_node_get_blockchain_info() {
        let exe = init();
        let node = Node::new(exe).unwrap();
        let info = node.client.get_blockchain_info().unwrap();
        assert_eq!(0, info.blocks);
    }

    #[test]
    fn test_node() {
        let exe = init();
        let node = Node::new(exe).unwrap();
        let info = node.client.get_blockchain_info().unwrap();

        assert_eq!(0, info.blocks);
        let address = node.client.new_address().unwrap();
        let _ = node.client.generate_to_address(1, &address).unwrap();
        let info = node.client.get_blockchain_info().unwrap();
        assert_eq!(1, info.blocks);
    }

    #[test]
    #[cfg(feature = "0_21_2")]
    fn test_getindexinfo() {
        let exe = init();
        let mut conf = Conf::default();
        conf.args.push("-txindex");
        let node = Node::with_conf(&exe, &conf).unwrap();
        assert!(
            node.client.server_version().unwrap() >= 210_000,
            "getindexinfo requires bitcoin >0.21"
        );
        let info: std::collections::HashMap<String, serde_json::Value> =
            node.client.call("getindexinfo", &[]).unwrap();
        assert!(info.contains_key("txindex"));
        assert!(node.client.server_version().unwrap() >= 210_000);
    }

    #[test]
    fn test_p2p() {
        let exe = init();

        let conf = Conf::<'_> { p2p: P2P::Yes, ..Default::default() };
        let node = Node::with_conf(&exe, &conf).unwrap();
        assert_eq!(peers_connected(&node.client), 0);

        let other_conf = Conf::<'_> { p2p: node.p2p_connect(false).unwrap(), ..Default::default() };
        let other_node = Node::with_conf(&exe, &other_conf).unwrap();

        assert_eq!(peers_connected(&node.client), 1);
        assert_eq!(peers_connected(&other_node.client), 1);
    }

    #[cfg(not(target_os = "windows"))] // TODO: investigate why it doesn't work in windows
    #[test]
    fn test_data_persistence() {
        // Create a Conf with staticdir type
        let mut conf = Conf::default();
        let datadir = TempDir::new().unwrap();
        conf.staticdir = Some(datadir.path().to_path_buf());

        // Start Node with persistent db config
        // Generate 101 blocks
        // Wallet balance should be 50
        let node = Node::with_conf(exe_path().unwrap(), &conf).unwrap();
        let core_addrs = node.client.new_address().unwrap();
        node.client.generate_to_address(101, &core_addrs).unwrap();
        let wallet_balance_1 = node.client.get_balance().unwrap();
        let best_block_1 = node.client.get_best_block_hash().unwrap();

        drop(node);

        // Start a new Node with the same datadir
        let node = Node::with_conf(exe_path().unwrap(), &conf).unwrap();

        let wallet_balance_2 = node.client.get_balance().unwrap();
        let best_block_2 = node.client.get_best_block_hash().unwrap();

        // Check node chain data persists
        assert_eq!(best_block_1, best_block_2);

        // Check the node wallet balance persists
        assert_eq!(wallet_balance_1, wallet_balance_2);
    }

    #[test]
    fn test_multi_p2p() {
        let exe = init();

        let conf_node1 = Conf::<'_> { p2p: P2P::Yes, ..Default::default() };
        let node1 = Node::with_conf(&exe, &conf_node1).unwrap();
        assert_eq!(peers_connected(&node1.client), 0);

        // Create Node 2 connected Node 1
        let conf_node2 = Conf::<'_> { p2p: node1.p2p_connect(true).unwrap(), ..Default::default() };
        let node2 = Node::with_conf(&exe, &conf_node2).unwrap();

        // Create Node 3 Connected To Node
        let conf_node3 =
            Conf::<'_> { p2p: node2.p2p_connect(false).unwrap(), ..Default::default() };
        let node3 = Node::with_conf(exe_path().unwrap(), &conf_node3).unwrap();

        // Get each nodes Peers
        let node1_peers = peers_connected(&node1.client);
        let node2_peers = peers_connected(&node2.client);
        let node3_peers = peers_connected(&node3.client);

        // Peers found
        assert!(node1_peers >= 1);
        assert!(node2_peers >= 1);
        assert_eq!(node3_peers, 1, "listen false but more than 1 peer");
    }

    #[cfg(feature = "0_19_1")]
    #[test]
    fn test_multi_wallet() {
        use corepc_client::bitcoin::Amount;

        let exe = init();
        let node = Node::new(exe).unwrap();
        let alice = node.create_wallet("alice").unwrap();
        let alice_address = alice.new_address().unwrap();
        let bob = node.create_wallet("bob").unwrap();
        let bob_address = bob.new_address().unwrap();
        node.client.generate_to_address(1, &alice_address).unwrap();
        node.client.generate_to_address(101, &bob_address).unwrap();

        let balances = alice.get_balances().unwrap();
        let alice_balances: vtype::GetBalances = balances;

        let balances = bob.get_balances().unwrap();
        let bob_balances: vtype::GetBalances = balances;

        assert_eq!(
            Amount::from_btc(50.0).unwrap(),
            Amount::from_btc(alice_balances.mine.trusted).unwrap()
        );
        assert_eq!(
            Amount::from_btc(50.0).unwrap(),
            Amount::from_btc(bob_balances.mine.trusted).unwrap()
        );
        assert_eq!(
            Amount::from_btc(5000.0).unwrap(),
            Amount::from_btc(bob_balances.mine.immature).unwrap()
        );
        let _txid = alice.send_to_address(&bob_address, Amount::from_btc(1.0).unwrap()).unwrap();

        let balances = alice.get_balances().unwrap();
        let alice_balances: vtype::GetBalances = balances;

        assert!(
            Amount::from_btc(alice_balances.mine.trusted).unwrap()
                < Amount::from_btc(49.0).unwrap()
                && Amount::from_btc(alice_balances.mine.trusted).unwrap()
                    > Amount::from_btc(48.9).unwrap()
        );

        // bob wallet may not be immediately updated
        for _ in 0..30 {
            let balances = bob.get_balances().unwrap();
            let bob_balances: vtype::GetBalances = balances;

            if Amount::from_btc(bob_balances.mine.untrusted_pending).unwrap().to_sat() > 0 {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        let balances = bob.get_balances().unwrap();
        let bob_balances: vtype::GetBalances = balances;

        assert_eq!(
            Amount::from_btc(1.0).unwrap(),
            Amount::from_btc(bob_balances.mine.untrusted_pending).unwrap()
        );
        assert!(node.create_wallet("bob").is_err(), "wallet already exist");
    }

    #[test]
    fn test_node_rpcuser_and_rpcpassword() {
        let exe = init();

        let mut conf = Conf::default();
        conf.args.push("-rpcuser=bitcoind");
        conf.args.push("-rpcpassword=bitcoind");

        let node = Node::with_conf(exe, &conf);

        assert!(node.is_err());
    }

    #[test]
    fn test_node_rpcauth() {
        let exe = init();

        let mut conf = Conf::default();
        // rpcauth generated with [rpcauth.py](https://github.com/bitcoin/bitcoin/blob/master/share/rpcauth/rpcauth.py)
        // this could be also added to node, example: [RpcAuth](https://github.com/testcontainers/testcontainers-rs/blob/dev/testcontainers/src/images/coblox_bitcoincore.rs#L39-L91)
        conf.args.push("-rpcauth=bitcoind:cccd5d7fd36e55c1b8576b8077dc1b83$60b5676a09f8518dcb4574838fb86f37700cd690d99bd2fdc2ea2bf2ab80ead6");

        let node = Node::with_conf(exe, &conf).unwrap();

        let auth = Auth::UserPass("bitcoind".to_string(), "bitcoind".to_string());
        let client = Client::new_with_auth(
            format!("{}/wallet/default", node.rpc_url().as_str()).as_str(),
            auth,
        )
        .unwrap();
        let info = client.get_blockchain_info().unwrap();
        assert_eq!(0, info.blocks);

        let address = client.new_address().unwrap();
        let _ = client.generate_to_address(1, &address).unwrap();
        let info = node.client.get_blockchain_info().unwrap();
        assert_eq!(1, info.blocks);
    }

    #[test]
    fn test_get_cookie_user_and_pass() {
        let exe = init();
        let node = Node::new(exe).unwrap();

        let user: &str = "bitcoind_user";
        let password: &str = "bitcoind_password";

        std::fs::write(&node.params.cookie_file, format!("{}:{}", user, password)).unwrap();

        let result_values = node.params.get_cookie_values().unwrap().unwrap();

        assert_eq!(user, result_values.user);
        assert_eq!(password, result_values.password);
    }

    #[test]
    fn zmq_interface_enabled() {
        let conf = Conf::<'_> { enable_zmq: true, ..Default::default() };
        let node = Node::with_conf(exe_path().unwrap(), &conf).unwrap();

        assert!(node.params.zmq_pub_raw_tx_socket.is_some());
        assert!(node.params.zmq_pub_raw_block_socket.is_some());
    }

    #[test]
    fn zmq_interface_disabled() {
        let exe = init();
        let node = Node::new(exe).unwrap();

        assert!(node.params.zmq_pub_raw_tx_socket.is_none());
        assert!(node.params.zmq_pub_raw_block_socket.is_none());
    }

    fn peers_connected(client: &Client) -> usize {
        // FIXME: Once client implements get_peer_info use it.
        // This is kinda cool, it shows we can call any RPC method using the client.
        let result: Vec<serde_json::Value> = client.call("getpeerinfo", &[]).unwrap();
        result.len()
    }

    fn init() -> String {
        let _ = env_logger::try_init();
        exe_path().unwrap()
    }
}
