It seems that it is not working.

## Usage

Small application of `wasm-pack`.

### Prepare

Install `wasm-pack`:

```shell
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Install npm: [How to Install Node.js and npm on Ubuntu 18.04 | Linuxize](https://linuxize.com/post/how-to-install-node-js-on-ubuntu-18.04/)

### Build with `wasm-pack build`

```shell
cd wasm-game-of-life
wasm-pack build
```

### launch the wasm-app(Conway's game of life)

```shell
cd wasm-game-of-life/www
npm install
cd wasm-game-of-life/pkg
npm link
cd wasm-game-of-life/www
npm link wasm-game-of-life
npm run start
```


