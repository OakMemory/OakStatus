use once_cell::sync::OnceCell;
use rocket::{
    figment::{
        providers::{Env, Format, Serialized, Toml},
        Figment, Profile,
    },
    tokio::sync::RwLock,
    Config,
};

use crate::utils::instance::OakSingleton;

pub type OakConfig = Figment;

impl OakSingleton for OakConfig {
    fn get_instance() -> &'static rocket::tokio::sync::RwLock<Self> {
        static INSTANCE: OnceCell<RwLock<OakConfig>> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            RwLock::new(
                OakConfig::from(rocket::Config::default())
                    .merge(Serialized::defaults(Config::default()))
                    .merge(Toml::file("Oak.toml").nested())
                    .merge(Env::prefixed("OAK_").global())
                    .select(Profile::from_env_or("OAK_PROFILE", "default")),
            )
        })
    }
}
