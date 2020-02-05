// https://github.com/kuflash/react-router-sitemap/blob/HEAD/api.md 
// https://babeljs.io/repl

// import React from "react"
// import { Switch, Route, } from "react-router"
// import Sitemap from 'react-router-sitemap';

// const sitemap = 
//  new Sitemap(<Route path='/home' />)
//    .build('http://www.steadylearner.com')
//    .save("./sitemap.xml")

// console.log(sitemap);

//

"use strict";

var _react = _interopRequireDefault(require("react"));

var _reactRouter = require("react-router");

var _reactRouterSitemap = _interopRequireDefault(require("react-router-sitemap"));

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

var sitemap = new _reactRouterSitemap.default(_react.default.createElement(_reactRouter.Route, {
  path: "/home"
})).build('http://www.steadylearner.com').save("./sitemap.xml");

console.log(sitemap);
