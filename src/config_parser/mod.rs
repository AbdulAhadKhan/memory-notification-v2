use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Config {
    pub core: Core,
    pub email: Email,
    pub policy_configs: PolicyConfigs,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Core {
    pub interval: u64,
    pub lower_limit: u64,
    pub upper_limit: u64,
    pub observation_window: usize,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Email {
    pub enabled: bool,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from: String,
    pub smtp_to: String,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct PolicyConfigs {
    pub p1_configs: P1Configs,
    pub p2_configs: P2Configs,
    pub p3_configs: P3Configs,
    pub p4_configs: P4Configs,
    pub p5_configs: P5Configs,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct P1Configs {
    pub enable_policy: bool,
    pub log_file: String,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct P2Configs {
    pub enable_policy: bool,
    pub delay: u64,
    pub log_file: String,
    pub enable_email: bool,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct P3Configs {
    pub enable_policy: bool,
    pub log_file: String,
    pub enable_email: bool,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct P4Configs {
    pub enable_policy: bool,
    pub log_file: String,
    pub enable_email: bool,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct P5Configs {
    pub enable_policy: bool,
    pub log_file: String,
    pub enable_email: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            core: Core::default(),
            email: Email::default(),
            policy_configs: PolicyConfigs::default(),
        }
    }
}

impl Default for Core {
    fn default() -> Self {
        Self {
            interval: 1,
            lower_limit: 5000,
            upper_limit: 10000,
            observation_window: 10,
        }
    }
}

impl Default for Email {
    fn default() -> Self {
        Self {
            enabled: false,
            smtp_server: "".to_string(),
            smtp_port: 0,
            smtp_username: "".to_string(),
            smtp_password: "".to_string(),
            smtp_from: "".to_string(),
            smtp_to: "".to_string(),
        }
    }
}

impl Default for PolicyConfigs {
    fn default() -> Self {
        Self {
            p1_configs: P1Configs::default(),
            p2_configs: P2Configs::default(),
            p3_configs: P3Configs::default(),
            p4_configs: P4Configs::default(),
            p5_configs: P5Configs::default(),
        }
    }
}

impl Default for P1Configs {
    fn default() -> Self {
        Self {
            enable_policy: true,
            log_file: "p1.log".to_string(),
        }
    }
}

impl Default for P2Configs {
    fn default() -> Self {
        Self {
            enable_policy: true,
            delay: 60,
            log_file: "p2.log".to_string(),
            enable_email: false,
        }
    }
}

impl Default for P3Configs {
    fn default() -> Self {
        Self {
            enable_policy: true,
            log_file: "p3.log".to_string(),
            enable_email: false,
        }
    }
}

impl Default for P4Configs {
    fn default() -> Self {
        Self {
            enable_policy: true,
            log_file: "p4.log".to_string(),
            enable_email: false,
        }
    }
}

impl Default for P5Configs {
    fn default() -> Self {
        Self {
            enable_policy: true,
            log_file: "p5.log".to_string(),
            enable_email: false,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let config = std::fs::read_to_string("config.toml").expect("Unable to read config file");
        toml::from_str(&config).expect("Unable to parse config file")
    }
}
