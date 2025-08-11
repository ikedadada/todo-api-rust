# Project Initialization Memo

1. use Rust.

```zsh
mise use rust@1.89.0
cargo init
```

2. add clippy.

```zsh
rustup component add clippy
(for CI ) cargo clippy --all-targets --all-features -- -D warnings
```

3. add rustfmt.

```zsh
rustup component add rustfmt
rustfmt --print-config default rustfmt.toml
```

4. add .gitignore.

```zsh
curl https://raw.githubusercontent.com/github/gitignore/main/Rust.gitignore -o .gitignore
```

5. add cargo-edit.

```zsh
cargo install cargo-edit
```

6. add rust-toolchain.toml.

```rust-toolchain.toml
[toolchain]
channel = "1.89.0"
components = ["clippy", "rustfmt"]
```
※ rust-toolchain.toml and mise.toml have active rust version. 

7. add Makefile.

```Makefile
# initialize
.PHONY: init
init:
	@echo "Initializing project..."
	cargo install cargo-edit
```