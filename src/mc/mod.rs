use std::env::current_dir;
use std::os::windows::process::CommandExt;
use crate::downloader::download_file;

const OFFICIAL_DOWNLOAD_LINK: &str = "https://launcher.mojang.com/download/Minecraft.exe";
const MC_FILE_PATH: &str = "./Minecraft/Launcher/Minecraft.exe";

pub struct Minecraft {}

impl Minecraft {
    pub async fn new() -> Self {
        let path = std::path::Path::new(MC_FILE_PATH);
        if !path.exists() {
            tokio::fs::create_dir_all(path.parent().unwrap()).await.expect("Cannot create dir");
        }

        Minecraft {}
    }

    pub async fn download_not_exists(&self, )
        -> Result<(), Box<dyn std::error::Error>> {
        let path = std::path::Path::new(MC_FILE_PATH);

        if !path.exists() {
            return download_file(String::from(OFFICIAL_DOWNLOAD_LINK), MC_FILE_PATH.to_owned()).await
        }

        Ok(())
    }

    pub async fn launch(&self, ) -> bool {
        let path = std::path::Path::new(MC_FILE_PATH);

        if !path.exists() {
            return false
        }

        let work_dir = current_dir().expect("Cannot get current dir")
            .join("Minecraft/.minecraft");
        let tmp_dir = current_dir().expect("Cannot get current dir")
            .join("Minecraft/Launcher").into_os_string();

        let spawned = std::process::Command::new(MC_FILE_PATH)
            .creation_flags(0x00000008)
            .raw_arg(format!("--workDir \"{}\"", String::from(work_dir.to_string_lossy())))
            .raw_arg(format!("--tmpDir \"{}\"", String::from(tmp_dir.to_string_lossy())))
            .spawn();

        match spawned {
            Ok(_) => {
                true
            },
            Err(_) => {
                false
            }
        }
    }
}