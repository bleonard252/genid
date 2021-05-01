# `genid`
A simple command-line tool written in Rust to generate a bunch of different types of IDs on the fly.

* :running: **Fast:** It's written in Rust so it's pretty quick
* :globe_with_meridians: **Unique:** Well, as long as you're using one of the unique ID types
* :gear: **Scriptable:** You can use this in scripts to make IDs for things

To get the previous behavior of genid's output (no newline), pass `--script`.

After cloning, you can use Cargo to run:
```sh
cargo run -q -- --help
```
(The `--` you see in the middle is necessary when running with Cargo. Otherwise, Cargo takes `--help` instead of genid.)

Or, after downloading a release binary, renaming it to `genid`, and adding it somewhere in your PATH (i.e. /usr/local/bin):
```sh
genid --help
```