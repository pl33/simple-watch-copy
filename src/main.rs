/*
 * SPDX-License-Identifier: MPL-2.0
 *   Copyright (c) 2021 Philipp Le <philipp@philipple.de>.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod config;

use std::{collections::HashMap, fs, format};
use inotify::{Inotify, WatchMask, WatchDescriptor};
use clap;
use simple_logger::SimpleLogger;
use log::{LevelFilter, info, warn, error};
use crate::config::{Config, WatchEntry};

fn main() {
    let args = clap::App::new("Simple Watch Copy")
        .version("1.0.0")
        .author("Philipp Le")
        .about("Watches directories and copies new files")
        .arg(clap::Arg::with_name("config")
            .short("c")
            .long("config")
            .takes_value(true)
            .help("Configuration JSON file"))
        .arg(clap::Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .takes_value(false)
            .help("Enable verbose output"))
        .get_matches();

    SimpleLogger::new().with_level(if args.is_present("verbose") {
        LevelFilter::Info
    } else {
        LevelFilter::Warn
    }).init().unwrap();

    let config_file = args.value_of("config").unwrap();
    let cfg = Config::read_from_file(config_file)
        .expect("Error while reading config file");

    let mut watcher = Inotify::init().expect("Failed to initialize inotify");

    let mut watches: HashMap<WatchDescriptor, WatchEntry> = HashMap::new();
    for ent in cfg.entries {
        let w = watcher.add_watch(
            &ent.src,
            WatchMask::CLOSE_WRITE
        ).expect("Failed to register watch");
        watches.insert(w, ent);
    }

    loop {
        let mut buffer = [0; 1024];
        let events = watcher.read_events_blocking(&mut buffer)
            .expect("Failed to retrieve inotify events");
        for event in events {
            let ent = match watches.get(&event.wd) {
                Some(val) => val,
                None => {
                    warn!("Could not find watch");
                    continue
                }
            };
            let filename = match event.name {
                Some(val) => match val.to_str() {
                    Some(s) => s,
                    None => {
                        warn!("Cannot decode name of inotify event");
                        continue
                    }
                },
                None => {
                    warn!("Received empty inotify event");
                    continue
                }
            };

            let src_path_buf = match fs::canonicalize(format!("{}/{}", ent.src, filename)) {
                Ok(val) => val,
                Err(e) => {
                    error!("Source file is not present, Error: {}", e);
                    continue;
                }
            };
            let src_path = src_path_buf.to_str().unwrap();
            info!("New or changed file: {}", src_path);
            for dest in &ent.dest {
                let dst_path = format!("{}/{}", dest, filename);
                match fs::copy(&src_path, &dst_path) {
                    Ok(len) => info!("Copied {} => {} ({} bytes)", src_path, dst_path, len),
                    Err(e) => error!("Failed to copy {} => {}: {}", src_path, dst_path, e)
                };
            }
        }
    }
}
