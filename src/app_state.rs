use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::apis::configuration::Configuration;

pub static CONFIG: Lazy<Mutex<Configuration>> = Lazy::new(|| Mutex::new(Configuration::default()));
