# Project Initialization Memo

1. use Rust.

```zsh
mise use rust@1.89.0
cargo init
```

2. add clippy.

```zsh
rustup component add clippy
cargo clippy

(for CI ) cargo clippy --all-targets --all-features -- -D warnings
```

3. add rustfmt.

```zsh
rustup component add rustfmt
cargo fmt
rustfmt --print-config default rustfmt.toml
```

4. add .gitignore.

```zsh
curl https://raw.githubusercontent.com/github/gitignore/main/Rust.gitignore -o .gitignore
```