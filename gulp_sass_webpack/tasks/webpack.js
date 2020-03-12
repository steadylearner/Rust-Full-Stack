// equal to webpack.config.js
import path    from 'path'
import webpack from 'webpack'
import process from 'process'

const isProduction = (process.env.NODE_ENV === 'production')

let config = {
    entry: './js/main.js',
    output: {
        filename: './js/bundle.js',
        path: path.resolve(__dirname, '../site')
    },
    context: path.resolve(__dirname, '../site'),
    plugins: isProduction ? [ new webpack.optimize.UglifyJsPlugin() ] : [],

    // The 'mode' option has not been set,
    // webpack will fallback to 'production' for this value.
    // Set 'mode' option to 'development' or 'production' to enable defaults for each environment.
    // Include it from the starting point not to show the error relevant to it.
    mode: isProduction ? 'production' : 'development',
}


function production() {

    return new Promise(resolve => webpack(config, (err, stats) => {

        if(err) console.log('Webpack', err)

        console.log(stats.toString({ /* stats options */ }))

        resolve()
    }))
}

module.exports = { config, production }
