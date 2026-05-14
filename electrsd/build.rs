fn main() { download::start().unwrap(); }

#[cfg(any(docsrs, not(feature = "download")))]
mod download {
    pub(crate) fn start() -> Result<(), ()> { Ok(()) }
}

#[cfg(feature = "download")]
mod download {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Cursor};
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;
    use std::str::FromStr;

    use bitcoin_hashes::{sha256, Hash};

    include!("src/versions.rs");

    const GITHUB_URL: &str =
        "https://github.com/RCasatta/electrsd/releases/download/electrs_releases";

    fn get_expected_sha256(filename: &str) -> anyhow::Result<sha256::Hash> {
        let file = File::open("sha256")?;
        for line in BufReader::new(file).lines().map_while(Result::ok) {
            let tokens: Vec<_> = line.split("  ").collect();
            if tokens.len() == 2 && filename == tokens[1] {
                return Ok(sha256::Hash::from_str(tokens[0]).unwrap());
            }
        }
        panic!("no sha256 entry for {} in electrsd/sha256", filename);
    }

    pub(crate) fn start() -> anyhow::Result<()> {
        if std::env::var_os("ELECTRSD_SKIP_DOWNLOAD").is_some() {
            return Ok(());
        }

        let download_filename_without_extension = electrs_name();
        let download_filename = format!("{}.zip", download_filename_without_extension);
        let expected_hash = get_expected_sha256(&download_filename)?;
        let out_dir = std::env::var_os("OUT_DIR").unwrap();
        let electrs_exe_home = Path::new(&out_dir).join("electrs");
        let destination_filename =
            electrs_exe_home.join(&download_filename_without_extension).join("electrs");

        if !destination_filename.exists() {
            println!("filename:{} version:{} hash:{}", download_filename, VERSION, expected_hash);

            let download_endpoint =
                std::env::var("ELECTRSD_DOWNLOAD_ENDPOINT").unwrap_or(GITHUB_URL.to_string());
            let url = format!("{}/{}", download_endpoint, download_filename);

            let downloaded_bytes = bitreq::get(url.clone()).send()?.into_bytes();

            let downloaded_hash = sha256::Hash::hash(&downloaded_bytes);
            assert_eq!(expected_hash, downloaded_hash, "expected hash of {} is not matching", url);
            let cursor = Cursor::new(downloaded_bytes);

            let mut archive = zip::ZipArchive::new(cursor)?;
            let mut file = archive.by_index(0)?;
            let parent = destination_filename.parent().unwrap();
            std::fs::create_dir_all(parent)?;
            let mut outfile = std::fs::File::create(&destination_filename)?;
            std::io::copy(&mut file, &mut outfile).unwrap();
            std::fs::set_permissions(
                &destination_filename,
                std::fs::Permissions::from_mode(0o755),
            )?;
        }
        Ok(())
    }
}
