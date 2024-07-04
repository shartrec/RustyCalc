/*
 * Copyright (c) 2024.
 *
 * Copyright 2024 Trevor Campbell
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
 * associated documentation files (the “Software”), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge, publish, distribute,
 * sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all copies or
 * substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
 * NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT
 * OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 */

#![allow(unused)]

use std::{
    str::FromStr,
    sync::{Arc, RwLock},
};

use lazy_static::lazy_static;
use log::{error, info, warn};
use preferences::{AppInfo, Preferences, PreferencesMap};

static PREFS_PATH: &str = "rusty-calc.config";
pub static APP_INFO: AppInfo = AppInfo {
    name: "kelpie-rust-calculator",
    author: "shartrec.com",
};

// Preference constants
pub static ANGLE_MODE: &str = "angle-mode";
pub static THEME: &str = "theme";

lazy_static! {
    static ref MANAGER: PreferenceManager = PreferenceManager {
        preferences: {
            match PreferencesMap::<String>::load(&APP_INFO, PREFS_PATH) {
                Ok(map) => Arc::new(RwLock::new(map)),
                Err(e) => {
                    warn!("Error opening preferences {}", e);
                    info!("A new preferences file will be created");
                    Arc::new(RwLock::new(PreferencesMap::new()))
                }
            }
        },
        path: PREFS_PATH,
    };
}

pub struct PreferenceManager {
    preferences: Arc<RwLock<PreferencesMap>>,
    path: &'static str,
}

impl PreferenceManager {
    pub fn get<T: FromStr>(&self, key: &str) -> Option<T> {
        match self.preferences.read().unwrap().get(key) {
            Some(s) => match s.parse::<T>() {
                Ok(i) => Some(i),
                Err(_e) => None,
            },
            None => None,
        }
    }
    pub fn put<T: ToString>(&self, key: &str, value: T) {
        {
            let mut prefs = self.preferences.write().unwrap();
            prefs.insert(key.to_string(), value.to_string());
        }
        self.store();
    }

    pub fn remove(&self, key: &str) {
        {
            let mut prefs = self.preferences.write().unwrap();
            let _e = prefs.remove(key);
        }
        self.store();
    }

    pub fn clear(&self) {
        {
            let mut prefs = self.preferences.write().unwrap();
            prefs.clear();
        }
        self.store();
    }

    fn store(&self) {
        let prefs = self.preferences.read().unwrap();
        let _ = prefs.save(&APP_INFO, self.path);
    }
}

pub fn manager() -> &'static PreferenceManager {
    &MANAGER
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, RwLock};

    use preferences::PreferencesMap;

    use crate::ui::preferences::PreferenceManager;

    #[test]
    fn test_save_restore() {
        let manager = PreferenceManager {
            preferences: Arc::new(RwLock::new(PreferencesMap::new())),
            path: "kelpie-unit-test",
        };

        manager.put("Test_KEY 1", "First");
        manager.put("Test_KEY 2", 1);
        manager.put("Test_KEY 3", 24.66);

        assert_eq!(
            manager.get::<String>("Test_KEY 1"),
            Some("First".to_string())
        );
        assert_eq!(manager.get::<i32>("Test_KEY 2"), Some(1));
        assert_eq!(manager.get::<f64>("Test_KEY 3"), Some(24.66));
    }
}
