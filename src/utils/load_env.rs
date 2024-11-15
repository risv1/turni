use viperus;

pub struct EnvConfig {
    pub port: u16,
    pub database_url: String,
}

pub fn load_config() -> EnvConfig {
    if let Ok(_) = viperus::load_file(".env", viperus::Format::ENV) {
        viperus::automatic_env(true);

        let mut missing_keys = Vec::new();

        let port: Option<i32> = viperus::get("PORT");
        let database_url: Option<String> = viperus::get("DATABASE_URL");

        if port.is_none() {
            missing_keys.push("PORT");
        }

        if database_url.is_none() {
            missing_keys.push("DATABASE_URL");
        }

        if !missing_keys.is_empty() {
            panic!("Missing keys in .env file: {:?}", missing_keys);
        }

        EnvConfig {
            port: port.unwrap() as u16,
            database_url: database_url.unwrap(),
        }
    } else {
        panic!("Missing .env file");
    }
}
