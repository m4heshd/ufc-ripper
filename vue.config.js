const {defineConfig} = require('@vue/cli-service');

module.exports = defineConfig({
    transpileDependencies: true,
    productionSourceMap: false,
    devServer: {
        port: 8383,
        devMiddleware: {
            writeToDisk: true
        }
    }
});
