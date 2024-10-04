const { defineConfig } = require("@vue/cli-service");
module.exports = defineConfig({
  transpileDependencies: true,
  chainWebpack: (config) => {
    config.module
      .rule("exclude-folder")
      .test("/src/helpers/blockchain/")
      .use("null-loader")
      .loader("null-loader");
  },
  lintOnSave: false,
  configureWebpack: (config) => {
    config.devtool = "source-map";
    config.resolve.symlinks = false;
    config.resolve.fallback = {
      crypto: require.resolve("crypto-browserify"),
      assert: require.resolve("assert"),
      stream: require.resolve("stream-browserify"),
    };
  },
});
