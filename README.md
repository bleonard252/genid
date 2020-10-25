# GenID
A simple command-line tool written in Rust to generate a bunch of different types of IDs on the fly.

* :person_running: **Fast:** It's written in Rust so it's pretty quick
* :globe_with_meridians: **Unique:** Well, as long as you're using one of the unique ID types
* :gear: **Scriptable:** You can use this in scripts to make IDs for things

Note that it doesn't output a new line. Zsh users will get an inverted `%` sign after the output, and Bash users will find their prompt after the generated ID.

After cloning, you can use Cargo to run:
```sh
cargo run -q -- --help
```
(The `--` you see in the middle is necessary when running with Cargo. Otherwise, Cargo takes `--help` instead of genid.)