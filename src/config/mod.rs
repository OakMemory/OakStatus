use once_cell::sync::OnceCell;
use rocket::{
    figment::{
        providers::{Env, Format, Serialized, Toml},
        Figment, Profile,
    },
    tokio::sync::RwLock,
};
use serde::{Deserialize, Serialize};

use crate::utils::instance::OakSingleton;

pub type OakConfig = Figment;

#[derive(Debug, Serialize, Deserialize)]
pub struct OakConfigContent {
    pub request_limit: usize,
    pub secret_bucket: Vec<String>,
    pub storage_time: usize,
}

impl Default for OakConfigContent {
    fn default() -> Self {
        Self {
            request_limit: 1000,
            secret_bucket: vec![
                r"J%(vae,q;8}WOZqG!a\Q".to_string(),
                r"hG|+@W00H:'30m9zW.j)".to_string(),
                r"*eKbnBl^RH.Sa997,Is".to_string(),
            ],
            storage_time: 6 * 60 * 60,
        }
    }
}

impl OakSingleton for OakConfig {
    fn get_instance() -> &'static rocket::tokio::sync::RwLock<Self> {
        static INSTANCE: OnceCell<RwLock<OakConfig>> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            RwLock::new(
                OakConfig::from(rocket::Config::default())
                    .merge(Serialized::defaults(OakConfigContent::default()))
                    .merge(Toml::file("Oak.toml").nested())
                    .merge(Env::prefixed("OAK_").global())
                    .select(Profile::from_env_or("OAK_PROFILE", "default")),
            )
        })
    }
}
