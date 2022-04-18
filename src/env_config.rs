pub struct EnvConfig {
    pub redis_host: String,
    pub redis_port: String,
}

pub fn load_env_config() -> EnvConfig {
    let redis_host = std::env::var("REDIS_HOST").unwrap_or("127.0.0.1".to_string());
    let redis_port = std::env::var("REDIS_PORT").unwrap_or("6379".to_string());
    let env_config = EnvConfig{
        redis_host,
        redis_port,
    };
    env_config
}