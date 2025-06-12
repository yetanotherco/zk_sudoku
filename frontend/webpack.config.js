const path = require('path');
const HtmlBundlerPlugin = require('html-bundler-webpack-plugin');

module.exports = {
    mode: 'production',
    output: {
        path: path.resolve(__dirname, 'dist'),
    },
    plugins: [
        new HtmlBundlerPlugin({
            entry: {
                index: 'src/index.html',
            },
            css: {
                inline: true, // inline CSS into HTML
            },
            js: {
                inline: true, // inline JS into HTML
            },
        }),
    ],
    module: {
        rules: [
            {
                test: /\.(js)$/,
                exclude: /node_modules/,
                use: [],
            },
            {
                test: /\.(css)$/,
                use: ['css-loader'],
            },
        ],
    },
    performance: false
};