fn main() {
    emit_resolved_version_cfgs();
    #[cfg(feature = "download")]
    download::download();
}

fn emit_resolved_version_cfgs() {
    let versions =
        ["electrs_0_10_6", "electrs_0_9_11", "electrs_0_9_1", "electrs_0_8_10", "esplora_a33e97e1"];

    // Puts all versions in the cfg namespace.
    // Tells rust these are valid and expected configs.
    for v in versions {
        println!("cargo:rustc-check-cfg=cfg({}_only)", v);
    }

    for v in versions {
        let env_name = format!("CARGO_FEATURE_{}", v.to_uppercase());
        if std::env::var_os(&env_name).is_some() {
            // Emits cfg for the highest enabled version only and then returns.
            println!("cargo:rustc-cfg={}_only", v);
            return;
        }
    }
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

    fn get_expected_sha256(filename: &str) -> Result<sha256::Hash, ()> {
        let file = File::open("sha256").map_err(|_| ())?;
        for line in BufReader::new(file).lines().map_while(Result::ok) {
            let tokens: Vec<_> = line.split("  ").collect();
            if tokens.len() == 2 && filename == tokens[1] {
                return sha256::Hash::from_str(tokens[0]).map_err(|_| ());
            }
        }
        Err(())
    }

    pub fn download() {
        if std::env::var_os("ELECTRSD_SKIP_DOWNLOAD").is_some() {
            return;
        }

        if !HAS_FEATURE {
            return;
        }
        let download_filename_without_extension = electrs_name();
        let download_filename = format!("{}.zip", download_filename_without_extension);
        dbg!(&download_filename);
        let expected_hash = get_expected_sha256(&download_filename).unwrap();
        let out_dir = std::env::var_os("OUT_DIR").unwrap();
        let electrs_exe_home = Path::new(&out_dir).join("electrs");
        let destination_filename =
            electrs_exe_home.join(&download_filename_without_extension).join("electrs");

        dbg!(&destination_filename);

        if !destination_filename.exists() {
            println!("filename:{} version:{} hash:{}", download_filename, VERSION, expected_hash);

            let download_endpoint =
                std::env::var("ELECTRSD_DOWNLOAD_ENDPOINT").unwrap_or(GITHUB_URL.to_string());
            let url = format!("{}/{}", download_endpoint, download_filename);

            let downloaded_bytes = bitreq::get(url).send().unwrap().into_bytes();

            let downloaded_hash = sha256::Hash::hash(&downloaded_bytes);
            assert_eq!(expected_hash, downloaded_hash);
            let cursor = Cursor::new(downloaded_bytes);

            let mut archive = zip::ZipArchive::new(cursor).unwrap();
            let mut file = archive.by_index(0).unwrap();
            std::fs::create_dir_all(destination_filename.parent().unwrap()).unwrap();
            let mut outfile = std::fs::File::create(&destination_filename).unwrap();

            std::io::copy(&mut file, &mut outfile).unwrap();
            std::fs::set_permissions(&destination_filename, std::fs::Permissions::from_mode(0o755))
                .unwrap();
        }
    }
}
