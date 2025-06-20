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
```bash
rustup doc
```
## View markdown with Zed
```
Ctrl+Shift+P → "Markdown: Open Preview" Or Cut command: `Ctrl+Shift+V`
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
## Init project with Git
```bash
git init
git add .
git commit -m "Init commit"
git branch -M main
git remote add origin git@github.com:Finc8888/hello-rust.git
git push -u origin main
```
