"use strict";

var webpack = require("webpack");

module.exports = function () {
  return {
    module: {
      // https://webpack.js.org/configuration/module/#condition exclude unecessary files
      rules: [{
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: {
          loader: "babel-loader",
          options: {
            presets: ['@babel/preset-env']
          }
        }
      }]
    },
    devServer: {
      historyApiFallback: true,
      proxy: {
        "/api": "http://localhost:8000",
        "/static": "http://localhost:8000"
      }
    },
    // url-loader -> Images, fonts etc
    // https://reactjs.org/blog/2018/09/10/introducing-the-react-profiler.html
    resolve: {
      // https://gist.github.com/bvaughn/25e6233aeb1b4f0cdb8d8366e54a3977
      alias: {
        "react-dom": "react-dom/profiling",
        "scheduler/tracing": "scheduler/tracing-profiling"
      }
    }
  };
};
