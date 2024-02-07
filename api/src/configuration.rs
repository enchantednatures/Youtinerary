use anyhow::Result;
use auth_config::AuthSettings;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub auth_settings: AuthSettings,
    pub app_settings: ApplicationSettings,
    pub redis_url: String,
}

impl Settings {
    pub fn load_config() -> Result<Self> {
        let base_path = std::env::current_dir()?;
        let configuration_directory = base_path.join("config");

        let environment: Environment = std::env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .unwrap_or(Environment::Local);

        let environment_filename = format!("{}.yaml", &environment.as_str());
        let settings = config::Config::builder()
            .add_source(config::File::from(
                configuration_directory.join("base.yaml"),
            ))
            .add_source(config::File::from(
                configuration_directory.join(environment_filename),
            ))
            // Add in settings from environment variables (with a prefix of APP and '__' as separator)
            // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;

        Ok(settings.try_deserialize::<Self>()?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationSettings {
    pub addr: [u8; 4],
    pub port: u16,
}

pub enum Environment {
    Development,
    Local,
    Staging,
    Production,
}

impl Environment {
    fn as_str(&self) -> &str {
        match self {
            Environment::Development => "dev",
            Environment::Local => "local",
            Environment::Staging => "staging",
            Environment::Production => "production",
        }
    }
}

impl From<&Environment> for String {
    fn from(val: &Environment) -> Self {
        match &val {
            Environment::Development => "development".into(),
            Environment::Local => "local".into(),
            Environment::Staging => "staging".into(),
            Environment::Production => "production".into(),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "local" => Ok(Environment::Local),
            "development" => Ok(Environment::Development),
            "staging" => Ok(Environment::Staging),
            "production" => Ok(Environment::Production),
            _ => Err(format!("Failed to parse{}", value)),
        }
    }
}
