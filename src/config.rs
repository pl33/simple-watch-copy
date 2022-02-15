/*
 * SPDX-License-Identifier: MPL-2.0
 *   Copyright (c) 2021 Philipp Le <philipp@philipple.de>.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{fs, error::Error};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WatchEntry {
    pub src: String,
    pub dest: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub version: i32,
    pub entries: Vec<WatchEntry>,
}

impl Config {
    pub fn read_from_file(filename: &str) -> Result<Config, Box<dyn Error>> {
        let json_str = fs::read_to_string(filename)?;
        let config = serde_json::from_str(&json_str)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use serde_json;

    #[test]
    fn simple_config() {
        let cfg_str = "{\
            \"version\": 1,\
            \"entries\": [\
                {\
                    \"src\": \"/src\",\
                    \"dest\": [\
                        \"/dest1\",\
                        \"/dest2\"\
                    ]\
                }\
            ]\
        }";
        let cfg: Config = serde_json::from_str(cfg_str).unwrap();
        assert_eq!(cfg.version, 1);
        assert_eq!(cfg.entries.len(), 1);
        assert_eq!(cfg.entries[0].src, "/src");
        assert_eq!(cfg.entries[0].dest.len(), 2);
        assert_eq!(cfg.entries[0].dest[0], "/dest1");
        assert_eq!(cfg.entries[0].dest[1], "/dest2");
    }
}
