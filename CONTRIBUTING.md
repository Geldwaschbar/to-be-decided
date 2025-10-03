# Contributing

## Build the game yourself

First, clone the game locally. You can do this by installing [Git](https://git-scm.com/downloads) and then running:

```sh
git clone https://github.com/Geldwaschbar/to-be-decided.git
```

To build the game, you need to have [Rust](https://rust-lang.org/tools/install/) installed.

If you have Nix installed, you can alternatively install all required packages in your development shell by running:

```sh
nix develop
```

Finally run:

```sh
cargo build
```

to install all packages. That's it!

If you want to build your own web deploy, please run the following commands:

```sh
rm -fr ./deploy # Remove old files
cargo build --release --target wasm32-unknown-unknown # Build WASM
mkdir -p ./deploy # Create deploy and copy all files
cp ./target/wasm32-unknown-unknown/release/to-be-decided.wasm ./deploy/
cp site/* ./deploy/
cp -r assets/ ./deploy/
cd deploy/ # Create zip file
zip -r -9 to-be-decided.zip ./*
```

If you want to build for linux, please run the following commands:

```sh
mkdir -p ./deploy
cargo build --release --target x86_64-unknown-linux-gnu
cp ./target/x86_64-unknown-linux-gnu/release/to-be-decided ./deploy/
```

## Commit Messages

Write your commit messages **in English only**! Please use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
Your commit messages should follow this structure:

- feat: add your cool new feature
- fix: fix a bug
- docs: add or improve documentation
- style: reformat code (no logic changes)

## Testing changes

You should **ALWAYS** test your changes before pushing them!
To test them locally, run the following command in your terminal:

```sh
cargo run
```
