# Bevy Game Jam 3

## Build

```bash
cargo build
```

### WASM
Build the WASM build:
```bash
$ rustup target install wasm32-unknown-unknown # only need to run this once
$ cargo build --release --target wasm32-unknown-unknown
$ wasm-bindgen --out-dir ./dist/ --target web .\target\wasm32-unknown-unknown\release\bevy-jam-game.wasm
```

The `./dist` directory will contain your js bundle and wasm binary.

To serve it locally you can use a simple web server. 
```bash
npx serve .
```

### itch.io
To distribute it to itch, you need to generate a `zip` where all of the `index.html` and the `assets` and `dist` folders are at the root of the archive. 

This can be done with a tool like 7zip or from powershell with:
```powershell
Compress-Archive -Path .\assets\,.\index.html,.\dist\ -DestinationPath bevy-jam-game.zip
```

Upload the archive to itch.io