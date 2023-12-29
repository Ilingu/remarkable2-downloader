# Remarkable Downaloader

#### Simple CLI script to download files from your remarkable

## Origin

Tired of the bad UX of `http://10.11.99.1/` that wouldn't let me download all the files for a complete backup, thus I reverse engineer their api _(which is open source at this point since there is 0 protections)_

## Installation

> Diclaimer, only the 'backup' subcommand works, too lazy to do the rest when I know I'll never use it

```bash
cargo build --release # must have cargo installed before imo
# executable can be found here:
./target/release/remarkable2-downloader
```

Have fun!

## Usage

```bash
remarkable2-downloader --help
```

## Made with

1. `Elegance` ~~
2. **No optimization in mind**
3. **Rust** 🦀
