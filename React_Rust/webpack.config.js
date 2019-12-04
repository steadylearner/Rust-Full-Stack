// Remove some documentations from the original file.
/* eslint-disable */

const webpack = require("webpack");

const HtmlWebPackPlugin = require("html-webpack-plugin"); // https://github.com/jantimon/html-webpack-plugin
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const CleanWebpackPlugin = require("clean-webpack-plugin");
const CSPWebpackPlugin = require("csp-webpack-plugin"); // allow you use CDN when there is security problem
const autoprefixer = require("autoprefixer");

// const DynamicCdnWebpackPlugin = require("dynamic-cdn-webpack-plugin"); // move dependencies to html

// const FilterWarningsPlugin = require('webpack-filter-warnings-plugin'); // To suppress warning from using react-easy-md
const TerserPlugin = require("terser-webpack-plugin"); // Use it instead of uglify https://stackoverflow.com/questions/47439067/uglifyjs-throws-unexpected-token-keyword-const-with-node-modules

// const path = require("path");
// function resolve (dir) {
//   return path.join(__dirname, '..', dir)
// };

// const MinifyPlugin = require("babel-minify-webpack-plugin");

// https://webpack.js.org/guides/environment-variables/
module.exports = (e) => {
	// console.log(e);

	const minimizer = [
		new TerserPlugin(),
	];

	let optimization = {
		minimizer,
		splitChunks: {
			cacheGroups: { // separate vendors
				commons: {
					test: /[\\/]node_modules[\\/]/,
					name: "vendors",
					chunks: "all",
				},
			},
		},
	};

	if (e !== undefined && e.cypress === true) {
		optimization = { minimizer };
	}

	let port = 8000;

	if (e !== undefined && e.aws === true) {
		// Make it work with AWS fargate and the Docker container for it.
		port = 80;
	}

	const server = `http://localhost:${port}`;
        // console.log(server); Test it with yarn build and yarn aws

	return {
		module: {

			// https://webpack.js.org/configuration/module/#condition exclude unecessary files
			rules: [
				{
					test: /\.(js|jsx)$/,
					exclude: /node_modules/,
					use: {
						loader: "babel-loader",
					},
				},
				{
					test: /\.(js)$/,
					exclude: /node_modules/,
				},
				{ // to remove source map error from 3rd libraires such as rxjs from the console.
					test: /\.(js|jsx)$/,
					use: ["source-map-loader"],
					enforce: "pre",
				},
				{
					test: /\.html$/,
					use: [
						{
							loader: "html-loader",
							options: { minimize: true },
						},
					],
				},
				{
					test: /\.(scss|css)$/,
					use: [
						MiniCssExtractPlugin.loader, // This plugin should be used only on production builds without style-loader
						{
							loader: "css-loader",
						},
						{
							loader: "postcss-loader",
							options: {
								autoprefixer: {
									browsers: ["last 2 versions"],
								},
								plugins: () => [
									autoprefixer,
								],
							},
						},
						{
							loader: "sass-loader",
							options: {},
						},
					],

					// It works with CSS, Sass, CSS Module, React Specific(Inline Style and styled components)
				},

				{
					test: /\.(png|jp(e*)g|svg)$/,
					use: [{
						loader: "url-loader",
						options: {
							limit: 50000, // Convert images less than 50kb
							// limit: 8000, // Convert images less than 8kb
							name: "images/[hash]-[name].[ext]",
						},
					}],
				},
			],
		},
		// It is important to include it to use react-easy-md without errors
		node: {
			fs: "empty",
			tls: "empty",
			net: "empty",
			child_process: "empty",
		},

		// historyApiFallback to handle browser unsync problem with webpack and react router
		// proxy to handle api CORS problem(https://webpack.js.org/configuration/dev-server/#devserver-proxy)

		devServer: {
			historyApiFallback: true,
			proxy: {
				"/api": server,
				"/static": server,
				"/public": server,
			},
		},

		// url-loader -> Images, fonts etc

		optimization,

		resolve: { // https://gist.github.com/bvaughn/25e6233aeb1b4f0cdb8d8366e54a3977
			alias: {
				"react-dom": "react-dom/profiling",
				"scheduler/tracing": "scheduler/tracing-profiling",
			},
		},

		plugins: [
			new webpack.ProgressPlugin(), // Show progress
			new webpack.optimize.AggressiveMergingPlugin(),
			new HtmlWebPackPlugin({ // https://github.com/jantimon/html-webpack-plugin
				template: "./src/index.html",
				filename: "./index.html",
			}),
			new CSPWebpackPlugin({
				"object-src": "\"none\"",
				"base-uri": "\"self\"",
				"script-src": [
					"\"unsafe-inline\"",
					"\"self\"",
					"\"unsafe-eval\"",
					"https://cdnjs.cloudflare.com/ajax/libs/babel-standalone/6.18.1/babel.min.js",
					"https://apis.google.com/",
				],
				"worker-src": ["\"self\"", "blob:"],
			}),
			new MiniCssExtractPlugin({
				filename: "[name].css",
				chunkFilename: "[id].css",
			}),
			new CleanWebpackPlugin(["public"]), // It inscreases process speed so much.

			// new DynamicCdnWebpackPlugin({ // unexpected results
			// 	exclude: ["rxjs"], // include package name here that shows errors, Id don't use redux-thunk anymore
			// 	// exclude: ["rxjs", "redux-thunk"], // include package name here that shows errors, Id don't use redux-thunk anymore
			// }),

			// To remove waring from using react-easy-md
			// new FilterWarningsPlugin({
			// 	exclude: /Critical dependency: the request of a dependency is an expression/,
			// }),
		],
	}
};
