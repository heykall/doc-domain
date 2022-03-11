const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === 'production',
      port: 8000
    },
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: "domain-documentation.js",
      webassemblyModuleFilename: "domain-documentation.wasm"
    },
    module: {
      rules: [
        {
          test: /\.(sass|less|css)$/,
          loaders: ['style-loader', 'css-loader', 'sass-loader']
        }
      ],
    },
    plugins: [
      new CopyWebpackPlugin({
        patterns: [
          { from: './static', to: distPath },
        ],
      }),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      })
    ],
    watch: argv.mode !== 'production'
  };
};
