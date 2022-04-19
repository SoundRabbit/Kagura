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
            template: path.join(__dirname, "./template/index.html"),
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
