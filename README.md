# wordle_solver
Wordle Game Solver

# Build & Run
```bash
cargo build --release; cargo run
```

## Dependencies

* To install Rust:
Go to https://rustup.rs/

* Diesel_CLI and sqlite-bundled.
```bash
cargo install diesel_cli --no-default-features --features sqlite-bundled
```
### On Windows
For Rust<br>
* Visual Studio C++ Build Tools 2015+<br>

For Diesel<br>
* sqlite3.lib<br>

### On Linux
**Alphine Linux have some issue working with Diesel**<br>

**For Rust**<br>

For Debian based:
```bash
sudo apt-get install build-essential -y
```
For Red Hat Based:
```bash
sudo dnf install cmake gcc -y
```


**For Diesel**<br>
`libsqlite3-dev`<br>

For Debian based:
```bash
sudo apt install libsqlite3-dev -y
```
For Red Hat Based:
```bash
sudo dnf install libsqlite3-dev -y
```
