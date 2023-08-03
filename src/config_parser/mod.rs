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
    pub email: String,
    pub password: String,
    pub recipient: String,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct PolicyConfigs {
    pub policy_1: P1Configs,
    pub policy_2: P2Configs,
    pub policy_3: P3Configs,
    pub policy_4: P4Configs,
    pub policy_5: P5Configs,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct P1Configs {
    pub enabled: bool,
    pub log_file: String,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct P2Configs {
    pub enabled: bool,
    pub delay: usize,
    pub log_file: String,
    pub enable_email: bool,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct P3Configs {
    pub enabled: bool,
    pub log_file: String,
    pub enable_email: bool,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct P4Configs {
    pub enabled: bool,
    pub log_file: String,
    pub enable_email: bool,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct P5Configs {
    pub enabled: bool,
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
            email: "".to_string(),
            password: "".to_string(),
            recipient: "".to_string(),
        }
    }
}

impl Default for PolicyConfigs {
    fn default() -> Self {
        Self {
            policy_1: P1Configs::default(),
            policy_2: P2Configs::default(),
            policy_3: P3Configs::default(),
            policy_4: P4Configs::default(),
            policy_5: P5Configs::default(),
        }
    }
}

impl Default for P1Configs {
    fn default() -> Self {
        Self {
            enabled: true,
            log_file: "policy_1.log".to_string(),
        }
    }
}

impl Default for P2Configs {
    fn default() -> Self {
        Self {
            enabled: true,
            delay: 60,
            log_file: "policy_2.log".to_string(),
            enable_email: false,
        }
    }
}

impl Default for P3Configs {
    fn default() -> Self {
        Self {
            enabled: true,
            log_file: "policy_3.log".to_string(),
            enable_email: false,
        }
    }
}

impl Default for P4Configs {
    fn default() -> Self {
        Self {
            enabled: true,
            log_file: "policy_4.log".to_string(),
            enable_email: false,
        }
    }
}

impl Default for P5Configs {
    fn default() -> Self {
        Self {
            enabled: true,
            log_file: "policy_5.log".to_string(),
            enable_email: false,
        }
    }
}

impl Config {
    fn build_cofig(config: &str) -> Config {
        let config: Config = toml::from_str(config).expect("Unable to parse config file");

        if config.core.lower_limit > config.core.upper_limit {
            println!("Lower limit must be less than upper limit");
            std::process::exit(1);
        }

        if config.core.observation_window <= 0 {
            println!("Observation window must be greater than 0");
            std::process::exit(1);
        }

        if config.core.interval <= 0 {
            println!("Interval must be greater than 0");
            std::process::exit(1);
        };

        if config.policy_configs.policy_2.delay > config.core.observation_window {
            println!("Delay for policy 2 must be greater than observation window");
            std::process::exit(1);
        }

        config
    }

    pub fn new(path: &str) -> Self {
        let config = std::fs::read_to_string(path);
        match config {
            Ok(config) => {
                println!("[*] Using config file: {}\n", path);
                Self::build_cofig(&config)
            }
            Err(_) => {
                println!("[-] Unable to read config file: {}", path);
                println!("[+] Using default config\n");
                Self::default()
            }
        }
    }
}
