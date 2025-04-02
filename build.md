
# Compilation

Pour lancer le jeu en graphique :
```shell
cargo run --package=board_graphic --release
```


version console :

```shell
cargo run --package=board_console --release
```

## Compiler au fur et à mesure des changements :

```shell
// Current html is release
// release mode (remove the `--release` flag for debug mode)

cargo watch --ignore '**/generated/**' -- cargo build --release --target wasm32-unknown-unknown --bin board_graphic


// base command
cargo watch -- cargo run
```

+ lancer le web serveur

## Compiler en une seule fois

```shell
cargo build --target wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown

cargo build --bin=board_graphic --package=board_graphic --release --target wasm32-unknown-unknown

cargo run --package=board_console --release
cargo run --package=board_graphic --release
```

## Create a new project in the workspace

```shell
cargo new NAME --lib
```

```shell
cargo run --package=board_graphic
```


### Pc

```shell
cargo run
cargo run --release
```

#### Documentation

Generate the documentation :
```rs
cargo doc
```

Also see `programming_convention.md` and `asset_building_process.md`


#### Compress the audio file / wav
See compress_wav_files.ps1
It use FFmpeg to compress the audio
Warnings : Sometime, compressed audio work on the web, but panic on windows

# Author
- Thomas Mewily 2024