# Show prayer times

This is a code example that uses the Al-Adhan API (https://aladhan.com/prayer-times-api#GetTimings) to fetch prayer timings for today for a specific location.

Mostly, though, it is an exercise in Rust.

## Usage

1. Install Rust (use `rust-up` from https://rustup.rs/)

3. Run `cargo --version` in a terminal as a sanity check to ensure that Rust is installed and available

2. Go inside the *show_prayer_times* directory and run the following from a terminal:

>$>cargo run -- -g <target_longitude> -a <target_latitude>