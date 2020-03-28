# Preparation

## necessaries

- cargo
- npm

## Install wasm-pack

```shell
cargo install wasm-pack
```

## create a crate

```shell
cargo new --lib crate_name
cd crate_name
```

## install webpack and so on

In this tutorial, you will compile your source codes using webpack, and use webpach-dev-server to show webpages. If you have a better idea, please send issue to my repogitory. Now, Kagura is developed by too small number of persons, so we welcome collaborators everytime.

```shell
npm init
npm install -D @wasm-tool/wasm-pack-plugin html-webpack-plugin webpack webpack-cli webpack-dev-server
```

## create webpack.config.js

Create `webpack.config.js` in root of crate, and write like this:

```javascript
const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    resolve: {
        extensions: [".js"]
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: path.join(__dirname, "./src/index.html")
        }),
        new WasmPackPlugin({
            crateDirectory: path.join(__dirname, "./")
        })
    ]
};
```

## create index.js and index.html

Create `index.js` and `index.html` in root of crate.

index.js：

```javascript
import("../pkg");
```

index.html:

```html
<!DOCTYPE html>
<html>

<head>
    <meta charset="utf-8" />
    <title>kagura tutorial</title>
</head>

<body>
    <div id="app"></div>
</body>

</html>
```

## modify package.json

Add start script to `package.json` in root of crate.

```json
"scripts": {
  "start": "./node_modules/.bin/webpack-dev-server"
},
```

## modify cargo.toml

Add `[dependencies]` and `[lib]` to `crago.toml` in root of crate. like this:

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
kagura = "0.8.5"
wasm-bindgen = "0.2"
```
