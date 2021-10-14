# Preparation

## Nessesary tools

### cargo

Probably, you already installed. But if you do not install yet, please see [here](https://www.rust-lang.org/learn/get-started).

### wasm-pack

You can install wasm-pack by this command:

```shell
$ cargo install wasm-pack
```

### Node.js and npm

You can get from [here](https://nodejs.org/ja/). If you want not to use npm but also to use yarn and so on, you can use it.

## File setting

You can create a cargo project by this command:

```shell
$ cargo new --lib crate-name
```

and, create sum files like [this](../../../project-template/^0.13/).

next: [hello world](./02_hello_world.md)

<div align="center">files</div>

```text
[crate-root]
┣.gitignore
┣Cargo.toml
┣package.json
┣webpack.config.js
┣[src]
┃┗lib.rs
┗[template]
　┣index.html
　┗index.js
```

---

<div align="center">.gitignore</div>

```text
/target
/node_modules
/pkg
```

---

<div align="center">Cargo.toml</div>

```toml
[package]
name = "kagura-project"
version = "0.1.0"
edition="2018"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
# with "isaribi", you can write style sheet like css-module or styled-component.
isaribi="^0.2"
js-sys="^0.3"
kagura="^0.13"
wasm-bindgen="^0.2"

[dependencies.web-sys]
version="^0.3"
features=[
# If you want to use HtmlCanvasElement, Blob, IdbDatabase and so on, 
# add features hear.
]
```

---

<div align="center">package.json</div>

```json
{
    "name": "kagura-template",
    "version": "0.1.0",
    "description": "template of kagura project for version ^0.13",
    "main": "index.js",
    "scripts": {
        "start": "./node_modules/.bin/webpack-dev-server",
        "make": "./node_modules/.bin/webpack"
    },
    "devDependencies": {
        "@wasm-tool/wasm-pack-plugin": "^1.6.0",
        "html-webpack-plugin": "^5.3.2",
        "webpack": "^4.58.1",
        "webpack-cli": "^4.9.0",
        "webpack-dev-server": "^4.3.1"
    }
}
```

---

<div align="center">webpack.config.js</div>

```js
const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    mode: "production",
    experiments: { syncWebAssembly: true },
    entry: "./template",
    output: {
        path: path.join(__dirname, "./dist")
    },
    resolve: {
        extensions: [".js"]
    },
    module: {
        rules: [],
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: path.join(__dirname, "./assets/index.html"),
            inlineSource: ".(js|css)$"
        }),
        new WasmPackPlugin({
            crateDirectory: path.join(__dirname, "./"),
            forceMode: "production",
            target: "web",
            args: "--log-level error",
        }),
    ],
    devServer: {
        historyApiFallback: true,
    }
};
```

---

<div align="center">src/lib.rs</div>

```rust
extern crate isaribi;
extern crate js_sys;
extern crate kagura;
extern crate wasm_bindgen;
extern crate web_sys;

use kagura::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    Kagura::mount(entry_point(), || {
        vec![Html::h1(
            Attributes::new(),
            Events::new(),
            vec![Html::text("Hello Kagura")],
        )]
    });
}

fn entry_point() -> web_sys::Node {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("app")
        .unwrap()
        .into()
}
```

---

<div align="center">template/index.html</div>

```html
<!DOCTYPE html>
<html>

<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Kagura</title>
</head>

<body>
    <div id="app">
    </div>
</body>

</html>
```

---

<div align="center">template/index.js</div>

```js
import("../pkg");
```