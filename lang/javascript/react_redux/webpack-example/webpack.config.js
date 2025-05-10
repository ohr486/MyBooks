module.exports = {
    mode: 'development',

    entry: './entry.js',

    output: {
        filename: 'output.js'
    },

    module: {
        rules: [
            {
                loader: 'babel-loader',
                test: /\.js$/,
                options: {
                    presets: [
                        '@babel/preset-react'
                    ]
                }
            }
        ]
    }
};