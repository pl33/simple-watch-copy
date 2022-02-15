# Simple Watch Copy

This application puts inotify watchers on directories, waits
until new files are placed there and then copies these files
to other directories.

## Build

### Cargo

Install the toolchain:

```shell
rustup target add x86_64-unknown-linux-gnu
```

Build the application:

```shell
cargo build --release
```

## Configuration

See [example_config.json](example_config.json)

## Run

### In Terminal

Usage: `./<BIN PATH>/simple_watch_copy --help`

### As systemd Service

1. Copy See [simple-watch-copy.service](simple-watch-copy.service) to `/home/<USER>/.config/systemd/system/`
2. Replace all paths with the correct ones.
3. `systemctl start --user simple-watch-copy.service`

`systemctl enable --user simple-watch-copy.service` to
permanently enable the service on system boot.
