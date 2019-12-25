use crate::models::common_models::{Db, Server, Verify};
use config::*;

#[allow(deprecated)]
lazy_static! {
    pub static ref DB: Db = {
        let mut config = Config::default();
        config.merge(File::with_name("settings/db.toml")).unwrap();
        let db: Db = config.try_into().unwrap();
        db
    };
}

#[allow(deprecated)]
lazy_static! {
    pub static ref SERVER: Server = {
        let mut config = Config::default();
        config.merge(File::with_name("settings/server.toml")).unwrap();
        let server: Server = config.try_into().unwrap();
        server
    };
}

#[allow(deprecated)]
lazy_static! {
    pub static ref VERIFY: Verify = {
        let mut config = Config::default();
        config.merge(File::with_name("settings/verify.toml")).unwrap();
        let verify: Verify = config.try_into().unwrap();
        verify
    };
}
