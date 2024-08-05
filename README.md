# rust-wasm-canvas-example

## WIP: STILL WORK IN PROGRESS. PLEASE WAIT.

## 1. About

Using WASM to draw fractals using Canvas API

## 2. Development

### 2-1. Format

#### A. clippy

Following NPM script will run `cargo clippy`:

```sh
npm run clippy
```

#### B. fmt

Following NPM script will format the codes:

```sh
npm run fmt
```

which is:

```sh
cargo +nightly fmt
```

For the above runs in the Nightly mode
because it has [rustfmt.toml](rustfmt.toml).

```toml
format_strings = true
```

### 2-2. Dev + Build

```sh
# DEV
npm run dev
# Check: http://localhost:8080

# BUILD
npm run build
```

## 3. Notes


## 4. What I did

### 4-1. Rust

See [Cargo.toml](./Cargo.toml) for installed Rust crates.

### 4-2. JS

#### Babel

- core-js
- @babel/cli
- @babel/core
- @babel/preset-env
- babel-loader

#### Webpack & Loaders

- webpack
- webpack-cli
- webpack-dev-server
- file-loader
- css-loader
- postcss-loader
- style-loader
- clean-webpack-plugin
- html-webpack-plugin
- autoprefixer

#### wasm-pack

- @wasm-tool/wasm-pack-plugin

#### Others

- postcss-cli
- postcss-preset-env
- postcss-import
- postcss-mixins
- tailwindcss
- nodemon
- concurrently
- rimraf
- prettier

#### All NPM Packages

```sh
npm install --save core-js

npm install --save-dev @babel/cli @babel/core @babel/preset-env \
  babel-loader webpack webpack-cli webpack-dev-server \
  file-loader css-loader postcss-loader style-loader \
  clean-webpack-plugin html-webpack-plugin \
  autoprefixer postcss-cli postcss-preset-env \
  postcss-import postcss-mixins \
  nodemon concurrently rimraf prettier \
  @wasm-tool/wasm-pack-plugin
```

## 5. License

Dual-licensed under MIT or the [UNLICENSE](https://unlicense.org/).  
Choose at your option.
