#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Config {
    pub db_path: String,
    pub tick_rate_ms: u64,
    pub theme: ThemeChoice,
}
impl Default for Config {
    fn default() -> Self {
        Self { db_path: "stash.db".to_string(), tick_rate_ms: 100, theme: ThemeChoice::Dark }
    }
}
impl Config {
    fn project_dirs() -> Option<directories::ProjectDirs> {
        directories::ProjectDirs::from("com", "stash", "stash-tui")
    }

    #[must_use]
    pub fn default_config_path() -> Option<std::path::PathBuf> {
        Self::project_dirs().map(|d| d.config_dir().join("config.toml"))
    }

    #[must_use]
    pub fn default_log_dir() -> Option<std::path::PathBuf> {
        // Prefer the XDG "state" dir for logs; fall back to the data dir on
        // platforms directories doesn't give us a state dir for.
        Self::project_dirs().map(|d| d.data_local_dir().join("logs"))
    }

    /// Loads config from `path`, or from the OS default location if `path`
    /// is `None`. Missing files silently fall back to `Config::default()` —
    /// a config file is a convenience, not a requirement.
    ///
    /// # Errors
    /// Returns an error only if a config file *exists* but fails to parse —
    /// a genuinely malformed file, unlike a missing one, is worth surfacing.
    pub fn load(path: Option<&std::path::Path>) -> anyhow::Result<Self> {
        let resolved = match path {
            Some(p) => Some(p.to_path_buf()),
            None => Self::default_config_path(),
        };

        let Some(resolved) = resolved else { return Ok(Self::default()) };
        if !resolved.exists() {
            return Ok(Self::default());
        }

        let raw = std::fs::read_to_string(&resolved)
            .map_err(|e| anyhow::anyhow!("failed to read config at {}: {e}", resolved.display()))?;
        toml::from_str(&raw)
            .map_err(|e| anyhow::anyhow!("invalid config at {}: {e}", resolved.display()))
    }

    /// Writes a default config.toml to `path` if nothing exists there yet —
    /// gives first-run users a template to edit instead of a blank slate.
    pub fn ensure_default_file(path: &std::path::Path) -> anyhow::Result<()> {
        if path.exists() {
            return Ok(());
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, toml::to_string_pretty(&Self::default())?)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ThemeChoice {
    Dark,
    Light,
}
