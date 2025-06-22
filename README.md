# Rust project
## Main commands:
### Create project with cargo
```bash
cargo new hello_cargo
```
### Build project with cargo
```bash
cargo build
```
### Update creates verisions with cargo(only minor versions, for major It's need to update Cargo.toml)
```bash
cargo update
```
*Build release (long compilation but optimized app)*
```bash
cargo build --release
```
### Run project with cargo
```bash
cargo run
```
### Check project with cargo
```bash
cargo check
```
### See Rust API in browser
Was a problem, opening other App instead browser. Fix in settings as choise Default App for Web(Set Google Chrome)
```bash
rustup doc
rustup doc --std
cargo doc --open
```
### Run examples
```bash
# Run particular example
cargo run --example array_vs_vec
cargo run --example simple_hello

# Run with arguments (if example takes it)
cargo run --example simple_hello -- arg1 arg2
```
### Run Tests
```bash
cargo test --examples
cargo test --example array_vs_vec
```
### See all commands
```bash
cargo help run
cargo run --help
```
## Zed shortcuts
```
Ctrl+Shift+P → "Markdown: Open Preview" Or Cut command: `Ctrl+Shift+V`
Ctrl+` - Open Terminal
Ctrl+J - Close Terminal
Ctrl+Shift+E - Open Project panel
Ctrl+B - Close Project Panel
F2 - Refactoring 'Rename variable'
Ctrl+, - Open settings.json file
```
## Run subprojects
### Guess gaming
```bash
cargo run --example guess_game
```
### Temperature converter
```bash
cargo run --example temperature_converter
```
### Fibonacci algorithm
```bash
cargo run --example fibonacci
```
## Init project with Git
```bash
git init
git add .
git commit -m "Init commit"
git branch -M main
git remote add origin git@github.com:Finc8888/hello-rust.git
git push -u origin main
```
