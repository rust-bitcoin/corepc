fn main() { download::start().unwrap(); }

#[cfg(not(feature = "download"))]
mod download {
    pub(crate) fn start() -> Result<(), ()> { Ok(()) }
}

#[cfg(feature = "download")]
#[cfg(not(docsrs))]
mod download {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader, Cursor, Read};
    use std::path::Path;
    use std::str::FromStr;

    use anyhow::Context;
    use bitcoin_hashes::{sha256, Hash};
    use flate2::read::GzDecoder;
    use tar::Archive;

    include!("src/versions.rs");

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    fn download_filename() -> String {
        if cfg!(not(feature = "23_2")) {
            format!("bitcoin-{}-osx64.tar.gz", &VERSION)
        } else {
            format!("bitcoin-{}-x86_64-apple-darwin.tar.gz", &VERSION)
        }
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    fn download_filename() -> String { format!("bitcoin-{}-arm64-apple-darwin.tar.gz", &VERSION) }

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    fn download_filename() -> String { format!("bitcoin-{}-x86_64-linux-gnu.tar.gz", &VERSION) }

    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    fn download_filename() -> String { format!("bitcoin-{}-aarch64-linux-gnu.tar.gz", &VERSION) }

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    fn download_filename() -> String { format!("bitcoin-{}-win64.zip", &VERSION) }

    #[allow(clippy::lines_filter_map_ok)] // clippy doesn't like the `lines` call below and the suggested fix is incorrect.
    fn get_expected_sha256(filename: &str) -> anyhow::Result<sha256::Hash> {
        let sha256sums_filename = format!("sha256/bitcoin-core-{}-SHA256SUMS", &VERSION);
        #[cfg(not(feature = "22_1"))]
        let sha256sums_filename = format!("{}.asc", sha256sums_filename);
        let file = File::open(&sha256sums_filename)
            .with_context(|| format!("cannot find {:?}", sha256sums_filename))?;
        for line in BufReader::new(file).lines().flatten() {
            let tokens: Vec<_> = line.split("  ").collect();
            if tokens.len() == 2 && filename == tokens[1] {
                return Ok(sha256::Hash::from_str(tokens[0]).unwrap());
            }
        }
        panic!(
            "Couldn't find hash for `{}` in `{}`:\n{}",
            filename,
            sha256sums_filename,
            std::fs::read_to_string(&sha256sums_filename).unwrap()
        );
    }

    pub(crate) fn start() -> anyhow::Result<()> {
        if std::env::var_os("BITCOIND_SKIP_DOWNLOAD").is_some() {
            return Ok(());
        }
        let download_filename = download_filename();
        println!("download_filename: {}", download_filename);
        let expected_hash = get_expected_sha256(&download_filename)?;
        let out_dir = std::env::var_os("OUT_DIR").unwrap();

        let mut bitcoin_exe_home = Path::new(&out_dir).join("bitcoin");
        if !bitcoin_exe_home.exists() {
            std::fs::create_dir(&bitcoin_exe_home)
                .with_context(|| format!("cannot create dir {:?}", bitcoin_exe_home))?;
        }
        let existing_filename =
            bitcoin_exe_home.join(format!("bitcoin-{}", VERSION)).join("bin").join("bitcoind");

        if !existing_filename.exists() {
            println!("filename:{} version:{} hash:{}", download_filename, VERSION, expected_hash);

            let (file_or_url, tarball_bytes) = match std::env::var("BITCOIND_TARBALL_FILE") {
                Err(_) => {
                    let download_endpoint = std::env::var("BITCOIND_DOWNLOAD_ENDPOINT")
                        .unwrap_or("https://bitcoincore.org/bin".to_owned());

                    let url = format!(
                        "{}/bitcoin-core-{}/{}",
                        download_endpoint, VERSION, download_filename
                    );
                    let resp = minreq::get(&url)
                        .send()
                        .with_context(|| format!("cannot reach url {}", url))?;
                    assert_eq!(resp.status_code, 200, "url {} didn't return 200", url);

                    (url, resp.as_bytes().to_vec())
                }
                Ok(path) => {
                    let f = File::open(&path).with_context(|| {
                        format!(
                            "Cannot find {:?} specified with env var BITCOIND_TARBALL_FILE",
                            &path
                        )
                    })?;
                    let mut reader = BufReader::new(f);
                    let mut buffer = Vec::new();
                    reader.read_to_end(&mut buffer)?;
                    (path, buffer)
                }
            };

            let tarball_hash = sha256::Hash::hash(&tarball_bytes);
            assert_eq!(
                expected_hash, tarball_hash,
                "expected hash of {} is not matching",
                file_or_url
            );

            if download_filename.ends_with(".tar.gz") {
                let d = GzDecoder::new(&tarball_bytes[..]);

                let mut archive = Archive::new(d);
                for mut entry in archive.entries().unwrap().flatten() {
                    if let Ok(file) = entry.path() {
                        if file.ends_with("bitcoind") {
                            entry.unpack_in(&bitcoin_exe_home).unwrap();
                        }
                    }
                }
            } else if download_filename.ends_with(".zip") {
                let cursor = Cursor::new(tarball_bytes);
                let mut archive = zip::ZipArchive::new(cursor).unwrap();
                for i in 0..zip::ZipArchive::len(&archive) {
                    let mut file = archive.by_index(i).unwrap();
                    let outpath = match file.enclosed_name() {
                        Some(path) => path.to_owned(),
                        None => continue,
                    };

                    if outpath.file_name().map(|s| s.to_str()) == Some(Some("bitcoind.exe")) {
                        for d in outpath.iter() {
                            bitcoin_exe_home.push(d);
                        }
                        let parent = bitcoin_exe_home.parent().unwrap();
                        std::fs::create_dir_all(parent)
                            .with_context(|| format!("cannot create dir {:?}", parent))?;
                        let mut outfile =
                            std::fs::File::create(&bitcoin_exe_home).with_context(|| {
                                format!("cannot create file {:?}", bitcoin_exe_home)
                            })?;
                        io::copy(&mut file, &mut outfile).unwrap();
                        break;
                    }
                }
            }

            // Code signing for arm64 macOS:
            #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
            {
                use std::process::Command;
                let status = Command::new("codesign")
                    .arg("-s")
                    .arg("-")
                    .arg(existing_filename.to_str().unwrap())
                    .status()
                    .with_context(|| "failed to execute codesign")?;
                if !status.success() {
                    return Err(anyhow::anyhow!(
                        "codesign failed with exit code {}",
                        status.code().unwrap_or(-1)
                    ));
                }
            }
        }
        Ok(())
    }
}
