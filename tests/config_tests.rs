use deckard_rs::config::{Config, DbConfig};

#[test]
fn save_and_load_config() {
    let mut cfg = Config::default();
    cfg.envs.insert(
        "test".to_string(),
        DbConfig {
            db_type: "postgres".to_string(),
            host: "localhost".to_string(),
            port: 5432,
            user: "user".to_string(),
            password: "pass".to_string(),
        },
    );
    cfg.save().unwrap();

    let loaded = Config::load().unwrap();
    assert!(loaded.envs.contains_key("test"));
}
