use viperus;

pub struct EnvConfig {
    pub port: u16,
}

pub fn load_config() -> EnvConfig {
    if let Ok(_) = viperus::load_file(".env", viperus::Format::ENV) {
        viperus::automatic_env(true);

        let mut missing_keys = Vec::new();

        let port: Option<i32> = viperus::get("PORT");

        if port.is_none() {
            missing_keys.push("PORT");
        }

        if !missing_keys.is_empty() {
            panic!("Missing keys in .env file: {:?}", missing_keys);
        }

        EnvConfig {
            port: port.unwrap() as u16,
        }
    } else {
        panic!("Missing .env file");
    }
}
