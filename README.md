<div align="center">

  <h1><code>wasm-pack-template</code></h1>

  <strong>A template for kick starting a Rust and WebAssembly project using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a>. Modified by <a href="https://github.com/Payne81">Payne81</a>.</sub>
</div>

## 🚴 Usage

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```
### 🐱‍🏍 launch the wasm-app(Conway's game of life) 

```
cd wasm-game-of-life/www
npm install
cd wasm-game-of-life/pkg
npm link
cd wasm-game-of-life/www
npm link wasm-game-of-life
npm run start
```

