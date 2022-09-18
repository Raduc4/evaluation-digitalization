const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const distPath = path.resolve(__dirname, "dist");
var mode = process.env.NODE_ENV || "development";
module.exports = {
  devtool: mode === "development" ? "inline-source-map" : false,
  mode: mode,

  devServer: {
    port: 8000,
  },
  entry: "./bootstrap.js",
  output: {
    path: distPath,
    filename: "index.js",
    webassemblyModuleFilename: "pkg.wasm",
    clean: true,
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [{ from: "./static", to: distPath }],
    }),
    new WasmPackPlugin({
      crateDirectory: ".",
      extraArgs: "-- --features wee_alloc",
      outName: "index",
      forceMode: mode,
      forceWatch: true,
      watchOptions: {
        aggregateTimeout: 200,
        poll: 200,
        //  is not necessary as long as you remove pkg/* before building your wasm files
      },
      watchDirectories: [
        // make distinct from outDir
        path.resolve(__dirname, "./src"), // point to rs src
      ],
    }),
  ],
  experiments: {
    syncWebAssembly: true,
  },
};
