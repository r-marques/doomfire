# DOOM Fire wasm

This example uses webassembly to run the DOOM Fire example in the browser.  The
example is mostly based on the [rustwasm Conway's Game of Life
tutorial](https://rustwasm.github.io/docs/book/game-of-life/introduction.html).

Most of the files are just boilerplate. The important files are:
- [`src/lib.rs`](https://github.com/r-marques/doomfire/blob/master/examples/doomfire-wasm/src/lib.rs) the rust code. A small wrapper around the `doomfire` library
- [`www/index.js`](https://github.com/r-marques/doomfire/blob/master/examples/doomfire-wasm/www/index.js) the javascript code that uses the canvas api to render the DOOM Fire
- [`www/index.html`](https://github.com/r-marques/doomfire/blob/master/examples/doomfire-wasm/www/index.html) the html page

### How to build and run

1. Make sure you have
   [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) and
   [`npm`](https://www.npmjs.com/get-npm) installed

2. Install the `wasm32-unknown-unknown` target

```bash
$ rustup target add wasm32-unknown-unknown
```
3. Compile rust to webassembly

```bash
$ cd examples/doomfire-wasm
$ wasm-pack build
```
4. Install `npm` dependencies

```bash
$ cd www
$ npm install
```

5. Run the development server

```bash
$ npm run start
```

6. Access [http://localhost:8080/](http://localhost:8080/) in your browser
