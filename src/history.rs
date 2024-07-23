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

use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::sync::RwLock;

use lazy_static::lazy_static;
use log::{info, warn};
use serde::{Deserialize, Serialize};

static HISTORY_FILE: &str = "rusty-calc-history.json";
static HISTORY_SIZE: usize = 100;

lazy_static! {
static ref HISTORY_MANAGER: HistoryManager = {
        let mut contents = String::new();
        let history = if let Some(path) = get_history_path() {
            match File::open(path)
                .and_then(|mut f| {
                    f.read_to_string(&mut contents)
                })
                {
                    Ok(_s) => {
                        serde_json::from_str(&contents).unwrap_or(History::new(HISTORY_SIZE))
                    }
                    Err(e) => {
                        warn!("Unable to open history file: {}", e);
                        info!("A new history file will be created");
                        History::new(HISTORY_SIZE)
                    }
                }
        } else {
            History::new(HISTORY_SIZE)
        };

        HistoryManager { history }
    };
}

pub struct HistoryManager {
     history: History,
}

impl HistoryManager {
    fn save_to_file(&self, path: PathBuf) -> io::Result<()> {
        let serialized = serde_json::to_string(&self.history)?;
        let mut file = File::create(&path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn save(&self) {
        if let Some(path) = get_history_path() {
            if let Err(e) = self.save_to_file(path) {
                warn!("{}", e.to_string());
                warn!("Failed to write history.")
            }
        }
    }
    pub fn add(&self, entry: (&str, &f64)) {
        self.history.add(entry);
    }

    pub fn history(&self) -> &History {
        &self.history
    }

}

pub fn manager() -> &'static HistoryManager {
    &HISTORY_MANAGER
}

fn get_history_path() -> Option<PathBuf> {
    home::home_dir().map( |home_path| {
        home_path.join(HISTORY_FILE)
    })
}
#[derive(Serialize, Deserialize, Debug)]
pub struct History {
    entries: RwLock<VecDeque<(String, f64)>>,
    #[serde(skip_serializing)]
    #[serde(default="History::defaut_size")]
    max_size: usize,
}

impl History {

    fn defaut_size() -> usize {
        HISTORY_SIZE
    }

    fn new(max_size: usize) -> Self {
        Self {
            entries: RwLock::new(VecDeque::with_capacity(max_size)),
            max_size,
        }
    }

    fn add(&self, entry: (&str, &f64)) {
        match self.entries.write() {
            Ok(mut vec) => {
                while vec.len() >= self.max_size {
                    vec.pop_back();
                }
                let new_entry = (entry.0.to_string(), entry.1.clone());
                vec.push_front(new_entry);
            }
            Err(_) => {
                warn!("Failed to write history.")
            }
        }
    }

    pub fn entries(&self) -> &RwLock<VecDeque<(String, f64)>> {
        &self.entries
    }

}