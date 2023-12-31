const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");

module.exports = {
	entry: "./bootstrap.js",
	output: {
		path: path.resolve(__dirname, "public"),
		filename: "bootstrap.js",
	},
	mode: "development",
	module: {
		rules: [
			{
				test: /\.tsx?$/,
				use: "ts-loader",
				exclude: /node_modules/,
			},
			{
				test: /\.css$/i,
				use: ["style-loader", "css-loader", "postcss-loader"],
			},
		],
	},
	resolve: {
		extensions: [".tsx", ".ts", ".js"],
	},
	plugins: [
		new CopyWebpackPlugin({
			patterns: [{ from: "./index.html", to: "./" }],
		}),
	],
	devServer: {
		watchFiles: ["./**/*.{ts,js,html}", "!node_modules"],
	},
};
