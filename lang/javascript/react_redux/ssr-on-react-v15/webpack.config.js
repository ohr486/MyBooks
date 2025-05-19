module.exports = {
    entry: './client.js',

    output: {
        filename: 'client.bundle.js'
    },

    mode: 'development',

    module: {
        rules: [
            {
                test: /\.js$/,
                loader: 'babel-loader',
                options: {
                    presets: ['@babel/preset-env', '@babel/preset-react']
                }
            }
        ]
    }
};
