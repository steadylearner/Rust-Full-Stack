// browserify npm.js > bundle.js
// They are executed when they are included with <script src="bundle.js"></script> in index.html file

const emoji = require("node-emoji");

// payload with browserify and later used for Rust Yew etc, you can use it directly with emoji
window.emoji = emoji; // you can use emoji or window.emoji in window(JavaScript global scope of index.html)

// console.log(emoji);
// console.log(emoji.emojify);
// console.log(emoji.emojify("I :heart: Rust - or use whatever you want"));

// const hello = () => console.log("Thank for using JavaScript in html. You can use it easily in browser with 'window.code = code' syntax");
// hello();

// window.hello = hello;
