use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::{env, fs};

use crate::Result;

pub const PORT: &str = "PORT";
pub const SSL: &str = "SSL";
pub const FRONTEND_DIR: &str = "FRONTEND_DIR";
pub const CAPTCHA_SITE_KEY: &str = "CAPTCHA_SITE_KEY";
pub const CAPTCHA_SITE_SECRET: &str = "CAPTCHA_SITE_SECRET";
pub const CLIENT_ID: &str = "CLIENT_ID";
pub const BUCKET_ID: &str = "BUCKET_ID";
pub const API_URL: &str = "API_URL";
pub const JWT_SECRET: &str = "JWT_SECRET";

#[derive(Clone, Deserialize)]
pub struct Config {
    pub port: u16,
    pub ssl: bool,
    pub frontend_dir: PathBuf,
    pub captcha_site_key: String,
    pub captcha_site_secret: String,
    pub client_id: String,
    pub bucket_id: String,
    pub api_url: String,
    pub jwt_secret: String,
    pub assets: AssetManifest,
}

#[derive(Clone, Deserialize)]
pub struct AssetManifest {
    pub main_js: String,
    pub vendor_js: String,
    pub gallery_js: String,
    pub upload_js: String,
    pub main_css: String,
    pub gallery_css: String,
}

#[derive(Deserialize)]
struct BundleConfig {
    suffix: String,
}

impl Config {
    pub fn build() -> Result<Config> {
        dotenv().ok();

        let env_port = env::var(PORT).expect("PORT is not set");
        let port: u16 = env_port.parse().expect("PORT is not a valid number");
        let env_ssl = env::var(SSL).expect("SSL is not set");
        let ssl = env_ssl.as_str() == "1";
        let env_frontend_dir = env::var(FRONTEND_DIR).expect("FRONTEND_DIR is not set");
        let frontend_dir = PathBuf::from(env_frontend_dir);
        let captcha_site_key: String =
            env::var(CAPTCHA_SITE_KEY).expect("CAPTCHA_SITE_KEY is not set");
        let captcha_site_secret: String =
            env::var(CAPTCHA_SITE_SECRET).expect("CAPTCHA_SITE_SECRET is not set");
        let client_id: String = env::var(CLIENT_ID).expect("CLIENT_ID is not set");
        let bucket_id: String = env::var(BUCKET_ID).expect("BUCKET_ID is not set");
        let api_url: String = env::var(API_URL).expect("API_URL is not set");
        let jwt_secret: String = env::var(JWT_SECRET).expect("JWT_SECRET is not set");

        if !frontend_dir.exists() {
            return Err("Frontend dir does not exists.".into());
        }

        let assets = AssetManifest::build(&frontend_dir)?;

        Ok(Config {
            port,
            ssl,
            frontend_dir,
            captcha_site_key,
            captcha_site_secret,
            client_id,
            bucket_id,
            api_url,
            jwt_secret,
            assets,
        })
    }
}

impl AssetManifest {
    pub fn build(frontend_dir: &PathBuf) -> Result<Self> {
        let filename = Path::new(frontend_dir).join("bundles.json");
        let Ok(contents) = fs::read_to_string(filename) else {
            return Err("Failed to read bundles.json".into());
        };
        let bundle_res = serde_json::from_str::<BundleConfig>(contents.as_str());
        let Ok(config) = bundle_res else {
            return Err("Failed to parse bundles.json".into());
        };

        Ok(AssetManifest {
            main_js: format!("/assets/bundles/js/main-{}.js", config.suffix),
            vendor_js: format!("/assets/bundles/js/vendor-{}.js", config.suffix),
            gallery_js: format!("/assets/bundles/js/gallery-{}.js", config.suffix),
            upload_js: format!("/assets/bundles/js/upload-{}.js", config.suffix),
            main_css: format!("/assets/bundles/css/main-{}.css", config.suffix),
            gallery_css: format!("/assets/bundles/css/gallery-{}.css", config.suffix),
        })
    }
}

/// memo-rs A photo gallery app
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Runs the web server
    Server,
}
