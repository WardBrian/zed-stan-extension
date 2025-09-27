use std::fs;

use zed::{Command, LanguageServerId, Result, Worktree};
use zed_extension_api::{self as zed};

struct StanExtension {
    cached_binary_path: Option<String>,
}

impl StanExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<String> {
        if let Some(binary_path) = worktree.which("stan-language-server") {
            return Ok(binary_path);
        };

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let (platform, arch) = zed::current_platform();

        let asset_name_ending = match (platform, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "macos-aarch64.zip",
            (zed::Os::Mac, zed::Architecture::X8664) => "macos-x86_64.zip",
            (zed::Os::Linux, zed::Architecture::Aarch64) => "linux-arm64.zip",
            (zed::Os::Linux, zed::Architecture::X8664) => "linux-x86_64.zip",
            (zed::Os::Windows, zed::Architecture::X8664) => "windows-x86_64.zip",
            _ => {
                return Err(format!(
                    "Unsupported platform or architecture: {:?}, {:?}",
                    platform, arch
                ))
            }
        };

        let release = zed::latest_github_release(
            "tomatitito/stan-language-server",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name.ends_with(asset_name_ending))
            .ok_or_else(|| format!("No asset found asset_name: {:?}", asset_name_ending))?;

        let version_dir = format!("stan-language-server/{}", release.version);
        let binary_path = match platform {
            zed::Os::Mac | zed::Os::Linux => format!("{version_dir}/stan-language-server"),
            zed::Os::Windows => format!("{version_dir}/stan-language-server.exe"),
        };

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|e| format!("failed to download file: {}", e))?;

            zed::make_file_executable(&binary_path)?;
        }

        Ok(binary_path)
    }
}

impl zed::Extension for StanExtension {
    fn new() -> Self {
        StanExtension {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        Ok(Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec!["--stdio".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(StanExtension);
