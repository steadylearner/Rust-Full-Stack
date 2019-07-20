// browserify npm.js > bundle.js
// They are executed when they are included with <script src="bundle.js"></script> in index.html file

const emoji = require("node-emoji");

// payload with browserify and later used for Rust Yew etc, you can use it directly with emoji
window.emoji = emoji; // you can use emoji or window.emoji in window(JavaScript global scope of index.html)
