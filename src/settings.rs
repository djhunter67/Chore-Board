use std::{
    collections::HashMap,
    convert::Infallible,
    env,
    future::{ready, Ready},
};

use actix_web::{web::Data, FromRequest, HttpRequest};
use serde::Deserialize;
use tracing::{info, instrument, warn};

/// Global setting for exposing all preconfigured variables
#[derive(Deserialize, Clone)]
pub struct Settings {
    pub application: Application,
    pub debug: bool,
    pub mongo: Mongo,
    pub redis: Redis,
    pub secret: Secret,
    pub email: Email,
    pub frontend_url: String,
    pub sqlite: Sqlite,
}

impl FromRequest for Settings {
    type Error = Infallible;

    type Future = Ready<Result<Self, Self::Error>>;

    #[instrument(
        name = "Settings from request",
        level = "info",
        target = "kid_data",
        skip(req)
    )]
    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let app_data = req.app_data::<Data<Self>>();
        let settings = app_data.expect("Settings not found in the request").clone();
        let var_name = settings.into_inner().as_ref().clone();
        ready(Ok(var_name))
    }

    fn extract(req: &actix_web::HttpRequest) -> Self::Future {
        Self::from_request(req, &mut actix_web::dev::Payload::None)
    }
}

#[derive(Deserialize, Clone)]
pub struct Doctor {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub speciality: String,
}

#[derive(Deserialize, Clone)]
pub struct Secret {
    pub secret_key: String,
    pub token_expiration: i64,
    pub hmac_secret: String,
}

#[derive(Deserialize, Clone)]
pub struct Email {
    pub host: String,
    pub host_user: String,
    pub host_user_password: String,
}

/// Redis setting for the entire application
#[derive(Deserialize, Clone, Debug)]
pub struct Redis {
    pub url: String,
    pub pool_max_open: u64,
    pub pool_max_idle: u64,
    pub pool_timeout_seconds: u64,
    pub pool_expire_seconds: u64,
}

/// Mongo setting for the entire application
#[derive(Deserialize, Clone, Debug)]
pub struct Mongo {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub db: String,
    pub collection: String,
    pub require_auth: bool,
}

/// Sqlite setting for the entire application
#[derive(Deserialize, Clone, Debug)]
pub struct Sqlite {
    pub db_path: String,
    pub pragma: HashMap<String, String>,
}

/// Application's specific settings to expose `port`,
/// `host`, `protocol`, and possible URL of the application
/// during and after development
#[derive(Deserialize, Clone)]
pub struct Application {
    pub port: u16,
    pub host: String,
    pub base_url: String,
    pub protocol: String,
}

/// The possible runtime environment for our application
pub enum Environment {
    Development,
    Production,
}

impl Environment {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    #[instrument(
        name = "Environment conversion",
        level = "info",
        target = "kid_data",
        skip(s)
    )]
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{other} is not a supported environment. Use either 'production' or 'development'."
            )),
        }
    }
}

/// # Result
///   - Returns a `Result` of `Settings` if successful
/// # Errors
///   - Returns a `config::ConfigError` if there is an error loading the settings
/// # Panics
///   - Panics if the current directory cannot be determined
/// # Notes
///   - Multipurpose function that helps detect the current environment the application
///     is running using the `APP_ENVIRONMENT` environment variable.
///
/// \\\
/// ``APP_ENVIRONMENT`` = development | production.
/// \\\
///
/// After detection, it loads the appropriate .yaml file
/// then it loads the environment variable that overrides whatever is set in the .yaml file.
/// For this to work, you the environment variable MUST be in uppercase and starts with `APP`,
/// a "_" separator then the category of settings,
/// followed by "__" separator,  and then the variable.
/// # Example
///   - ``APP__APPLICATION_PORT=5001`` for "port" to be set as "5001"
#[instrument(name = "Get settings", level = "info", target = "kid_data")]
pub fn get() -> Result<Settings, config::ConfigError> {
    info!("Getting the system config settings");
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    warn!(
        "the contents of the settings file: {:?}",
        base_path.join("settings")
    );
    let setting_directory = base_path.join("settings");

    let environment: Environment = match env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| String::from("development"))
        .try_into()
    {
        Ok(env) => env,
        Err(err) => return Err(config::ConfigError::Message(err)),
    };
    let environment_filename = format!("{}.yaml", environment.as_str());
    warn!(
        "Building the settings for the {} environment",
        environment.as_str().to_uppercase()
    );
    let settings = config::Config::builder()
        .add_source(config::File::from(
            setting_directory.join(environment_filename),
        ))
        .add_source(config::File::from(setting_directory.join("base.yaml")))
        // Add in settings from environment variables (with a prefix of APP and '__' as seperator)
        // e.g `APP_APPLICATION__PORT_5001 would set `Setting.application.port`
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_settings() {
        let settings = get().expect("Failed to get settings");
        assert!(settings.debug);
    }

    #[test]
    fn test_environment_try_from() {
        let dev_env = Environment::try_from("development".to_string()).unwrap();
        assert_eq!(dev_env.as_str(), "development");

        let prod_env = Environment::try_from("production".to_string()).unwrap();
        assert_eq!(prod_env.as_str(), "production");

        let invalid_env = Environment::try_from("staging".to_string());
        assert!(invalid_env.is_err());
    }

    // assert that the redis settings are correctly deserialized
    #[test]
    fn test_redis_settings() {
        let settings = get().expect("Failed to get settings");

        let redis_settings = settings.redis;

        assert!(!redis_settings.url.is_empty());
        assert!(redis_settings.pool_max_open > 0);
        assert!(redis_settings.pool_max_idle > 0);
        assert!(redis_settings.pool_timeout_seconds > 0);
        assert!(redis_settings.pool_expire_seconds > 0);
    }
}
