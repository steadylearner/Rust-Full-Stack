// eslint-disable-next-line no-unused-vars
import React from "react";
import styled from "styled-components";
import animated from "./animated";

const SteadylearnerCSS = styled.section`

/* I will use some of CSS from my website at the start of the development. */

/*

The MIT License (MIT)

Copyright (c) steadylearenr(https://www.steadylearner.com)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

*/

.heart {
  position: relative;
  width: 4.5rem;
  height: 4.5rem;
}

.heart::before, .heart::after {
  position: absolute;
  content: "";
  left: 2.5rem;
  top: 0;
  width: 2.5rem;
  height: 4rem;
  background: red;
  border-radius: 2.5rem 2.5rem 0 0;
  transform: rotate(-45deg);
  transform-origin: 0 100%;
}

.heart::after {
  left: 0;
  transform: rotate(45deg);
  transform-origin: 100% 100%;
}

.heart--youtube {
  z-index: 1;
  font-size: 2rem;
  position: absolute;
  color: white;
  margin-left: 1.6rem;
  margin-top: 0.8rem;
}

.YouTube-heart-image {
  width: 5.6rem;
  /* margin-bottom: 0.5rem; */
}

.YouTube-cursor {
  cursor: url("https://www.steadylearner.com/static/images/brand/Steadylearner_YouTube_s.png"), auto;
}

/* content */

.default-padding {
  padding: 0.5rem;
}

/* Remove this later */

.default-padding-small {
  padding: 0.1rem;
}

.padding-a-point {
  padding: 0.1rem;
}

.padding-two-point {
  padding: 0.2rem;
}

.padding-left-a-quarter {
  padding-left: 0.25rem;
}

.padding-right-a-quarter {
  padding-right: 0.25rem;
}

.padding-top-a-quarter {
  padding-top: 0.25rem;
}

.padding-bottom-a-quarter {
  padding-bottom: 0.25rem;
}

.padding-left-a-point {
  padding-left: 0.1rem;
}

.padding-right-a-point {
  padding-right: 0.1rem;
}

.padding-top-a-point {
  padding-top: 0.1rem;
}

.padding-bottom-a-point {
  padding-bottom: 0.1rem;
}

/* font */

.font-one {
  font-size: 1rem;
}

.font-one-quarter {
  font-size: 1.25rem;
}

.font-one-and-a-half {
  font-size: 1.5rem;
}

.font-normal {
  font-size: 1.6rem;
}

.font-one-and-eight {
  font-size: 1.8rem;
}

.font-two {
  font-size: 2rem;
}

.font-two-and-a-half {
  font-size: 2.5rem;
}

.font-two-and-eight {
  font-size: 2.8rem;
}

.font-three-and-a-point {
  font-size: 3.1rem;
}

.font-three-and-four {
  font-size: 3.4rem;
}

.font-three-and-six {
  font-size: 3.6rem;
}

.font--helvetica {
  font-family: Arial, Helvetica, sans-serif;
}

.font--cursive {
  font-family: cursive;
}

.bold {
  font-weight: bold;
}

.uppercase {
  text-transform: uppercase;
}

/* color */

.red {
  color: red;
}

.red-white {
  color: #ff7676;
}

.blue {
  color: #08c;
}

.brown {
  color: brown;
}

.green {
  color: green;
}

.yellow {
  color: yellow;
}

.yellow-green {
  color: yellowgreen;
}

.black {
  color: #121212;
}

/* Letter Space */

.letter-space--five-point {
  letter-spacing: 0.05rem;
}

/* Background for colors */

.theme-black {
  background-color: #121212;
}

.theme-white {
  background-color: white;
}

.theme-black-white {
  background-color: #efefef;
}

.theme-green {
  background-color: green;
}

.theme-red {
  background-color: red;
}

.theme-blue {
  background-color: #08c;
}

.theme-youtube {
  background-color: #bb0000;
}

/* background-images */

.theme--black-white-crossed-line {
  background: url("/static/images/tile/black_white_crossed_lines.jpg");
}

.theme--black-white-relevant {
  background: url("/static/images/tile/ef_relevant.jpg");
}

.theme--pink-pattern {
  background: url("/static/images/tile/pink_pattern.jpg");
}

.theme--green-crossed-lines {
  background: url("/static/images/tile/green_crossed_lines.jpg");
}

.theme--blue-square {
  background: url("/static/images/tile/blue_square.jpg");
}

.theme--blue-tile {
  background: url("/static/images/tile/blue_tile.jpg");
}

.theme--blue-burn {
  background: url("/static/images/tile/blue_burn.jpg");
}

/*  */

.white {
  color: #efefef;
}

.border-white {
  border: 1px solid rgba(238, 238, 238, 1);
}

/* no series -> to x later*/

.no-text-decoration {
  text-decoration: none;
}

.no-list-style {
  list-style: none;
  list-style-type: none;
}

/* Don't maintain layout */

.inherit-display {
  display: inherit;
}

.x-display {
  display: none !important;
}

/* It maintains layout */

.x-opacity {
  opacity: 0;
}

.x-outline {
  outline: none;
}

.x-overflow {
  overflow: hidden;
}

.x-border-style {
  border-style: none;
}

.x-margin {
  margin: 0;
}

.x-padding {
  padding: 0;
}

.x-ul {
  list-style: none;
  list-style-type: none;
  margin: 0;
  padding: 0;
}

.x-default-file-input {
  width: 0.1px;
  height: 0.1px;
  opacity: 0;
  overflow: hidden;
  position: absolute;
  z-index: -1;
}

/* effect */

.hover:hover {
  opacity: 0.7;
}

.hover-show {
  opacity: 0.1;
  :hover {
    opacity: 1;
  }
}

.underline {
  text-decoration: underline;
}

.underline-hover:hover {
  text-decoration: underline;
  color: #121212;
}

.transition {
  transition: 1s;
}

.transition-half {
  transition: 0.5s;
}

.transition--ease-in-and-out {
  transition: ease-in-out 1s;
}

.transition--ease-in-and-out-half {
  transition: ease-in-out 0.5s;
}

.scale-a-point {
  transform: scale(1.01);
}

.scale-five-point {
  transform: scale(1.05);
}

.scale-fifteen-point {
  transform: scale(1.15);
}

.scale-a-point--hover:hover {
  transform: scale(1.01);
}

.scale-five-point--hover:hover {
  transform: scale(1.05);
}

.scale-fifteen-point--hover:hover {
  transform: scale(1.15);
}

.cursor-pointer {
  cursor: pointer;
}

.cursor-auto {
  cursor: auto;
}

.cursor-default {
  cursor: auto;
}

.text-shadow-white {
  text-shadow: 0 0 0.5rem white;
}

.text-shadow-white-black {
  text-shadow: 0 0 0.5rem #efefef;
}

.text-shadow-black {
  text-shadow: 0 0 0.5rem #121212;
}

.text-shadow-black-hover:hover {
  text-shadow: 0 0 1rem #121212;
}

.text-shadow-black-hover--l:hover {
  text-shadow: 0 0 0.2rem #121212;
}

.text-shadow-blue-hover--l:hover {
  text-shadow: 0 0 0.2rem #08c;
}

.text-shadow-blue-hover:hover {
  text-shadow: 0 0 1rem #08c;
}

.text-shadow-red-hover:hover {
  text-shadow: 0 0 1rem #ff7676;
}

.text-shadow-red-hover--l:hover {
  text-shadow: 0 0 0.2rem #ff7676;
}

.text-shadow-yellow:hover {
  text-shadow: 0 0 1rem yellow;
}

.text-shadow-yellow-l:hover {
  text-shadow: 0 0 0.2rem yellow;
}

.link--active-yellow {
  text-shadow: 0 0 1rem yellow;
}

.link--active-blue {
  text-shadow: 0 0 1rem #08c;
}

.link--active-red {
  text-shadow: 0 0 1rem #ff7676;
}

.thumbnail {
  transition: transform 1s ease;
  :hover {
    opacity: 0.8;
    box-shadow: 0 0 0.5rem rgba(0, 130, 176, 0.8);
    cursor: pointer;
  }
}

/* position */

.fixed {
  position: fixed;
}

.absolute {
  position: absolute;
}

.sticky {
  position: sticky;
}

.relative {
  position: relative;
}

.index-one {
  z-index: 1;
}

/* layout */

.content__main {
  flex: 1 1 auto;
  max-width: 64rem;
  width: 64rem;
  margin-left: 3rem;
  margin-right: 6rem;
  margin-bottom: 1rem;
  padding: 0 2rem;
}

.content__side {
  flex: 0 0 auto;
  margin-right: 3rem;
}

.margin-left-a-point {
  margin-left: 0.1rem;
}

.margin-right-a-point {
  margin-right: 0.1rem;
}

.margin-top-a-point {
  margin-top: 0.1rem;
}

.margin-bottom-a-point {
  margin-bottom: 0.1rem;
}

.margin-left-two-point {
  margin-left: 0.2rem;
}

.margin-right-two-point {
  margin-right: 0.2rem;
}

.margin-top-two-point {
  margin-top: 0.2rem;
}

.margin-bottom-two-point {
  margin-bottom: 0.2rem;
}

.margin-left-a-quarter {
  margin-left: 0.25rem;
}

.margin-right-a-quarter {
  margin-right: 0.25rem;
}

.margin-top-a-quarter {
  margin-top: 0.25rem;
}

.margin-bottom-a-quarter {
  margin-bottom: 0.25rem;
}

.margin-left-half {
  margin-left: 0.5rem;
}

.margin-right-half {
  margin-right: 0.5rem;
}

.margin-top-half {
  margin-top: 0.5rem;
}

.margin-bottom-half {
  margin-bottom: 0.5rem;
}

.margin-left-one {
  margin-left: 1rem;
}

.margin-right-one {
  margin-right: 1rem;
}

.margin-top-one {
  margin-top: 1rem;
}

.margin-bottom-one {
  margin-bottom: 1rem;
}

.margin-left-one-and-a-half {
  margin-left: 1.5rem;
}

.margin-right-one-and-a-half {
  margin-right: 1.5rem;
}

.margin-top-one-and-a-half {
  margin-top: 1.5rem;
}

.margin-bottom-one-and-a-half {
  margin-bottom: 1.5rem;
}

.margin-left-two {
  margin-left: 2rem;
}

.margin-right-two {
  margin-right: 2rem;
}

.margin-top-two {
  margin-top: 2rem;
}

.margin-bottom-two {
  margin-bottom: 2rem;
}

.margin-left-two-and-a-half {
  margin-left: 2.5rem;
}

.margin-right-two-and-a-half {
  margin-right: 2.5rem;
}

.margin-top-two-and-a-half {
  margin-top: 2.5rem;
}

.margin-bottom-two-and-a-half {
  margin-bottom: 2.5rem;
}

.margin-left-four {
  margin-left: 4rem;
}

.margin-right-four {
  margin-right: 4rem;
}

.margin-top-four {
  margin-top: 4rem;
}

.margin-bottom-four {
  margin-bottom: 4rem;
}

.padding-left-four {
  padding-left: 4rem;
}

.padding-right-four {
  padding-right: 4rem;
}

.padding-top-four {
  padding-top: 4rem;
}

.padding-bottom-five {
  padding-bottom: 5rem;
}

.padding-left-five {
  padding-left: 5rem;
}

.padding-right-five {
  padding-right: 5rem;
}

.padding-top-five {
  padding-top: 5rem;
}

.padding-bottom-four {
  padding-bottom: 5rem;
}

.padding-left-fifteen {
  padding-left: 15rem;
}

.padding-right-fifteen {
  padding-right: 15rem;
}

.padding-top-fifteen {
  padding-top: 15rem;
}

.padding-bottom-fifteen {
  padding-bottom: 15rem;
}

.text-init {
  margin-left: 1rem;
}

.text-end {
  margin-right: 1rem;
}

.text-center {
  text-align: center;
}

.border {
  border-radius: 0.5rem;
  /* border-radius: 0.2rem; */
}

.border-round {
  border-radius: 50%;
}

.border-top-large-black {
  border-top: 0.6rem solid #121212;
}

.border-left-large-black {
  border-left: 0.6rem solid #121212;
}

.border-top-large-blue {
  border-top: 0.6rem solid #08c;
}

.border-left-large-blue {
  border-left: 0.6rem solid #08c;
}

.border-top-yellow {
  border-top: 0.2rem solid yellow;
}

.border-left-yellow {
  border-left: 0.2rem solid yellow;
}

.border-top-blue {
  border-top: 0.2rem solid #08c;
}

.border-left-blue {
  border-left: 0.2rem solid #08c;
}

.hover-round:hover {
  border-radius: 50%;
}

.width-center--default {
  width: 65rem;
}

.width-two-and-a-half {
  width: 2.5rem;
}

.width-four {
  width: 4rem;
}

.width-five {
  width: 5rem;
}

.width-five-point-six {
  width: 5, 6rem;
}

.width-vw {
  width: 100vw;
}

.width-percent {
  width: 100%;
}

.height-percent {
  height: 100%;
}

.min-height-percent {
  min-height: 100%;
}

.min-height-vh {
  min-height: 100vh;
}

.height-vh {
  height: 100vh;
}

.height-two-and-a-half {
  height: 2.5rem;
}

.max-width {
  max-width: 100%;
}

.max-width-half {
  max-width: 50%;
}

.max-width-sixty-four {
  max-height: 64rem;
}

.max-height-thirty-six {
  max-height: 36rem;
}

.left-auto {
  margin-left: auto;
}

.right-auto {
  margin-right: auto;
}

.center {
  display: flex;
  justify-content: center;
  align-items: center;
}

.center-percent-absolute {
  position: absolute;
  margin: 0;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.center-percent-relative {
  position: relative;
  margin: 0;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.flex-flow-row {
  flex-flow: row;
}

.flex-flow-column {
  flex-flow: column;
}

.column-center {
  display: flex;
  flex-flow: column wrap;
  justify-content: center;
  align-items: center;
}

.column-center-start {
  display: flex;
  flex-flow: column wrap;
  justify-content: center;
}

.background-cover {
  background-size: cover;
  background-repeat: no-repeat;
  background-position: center;
}

.background-percent {
  background-position: center;
  background-repeat: no-repeat;
  background-size: 100% 100%;
}

.no-repeat {
  background-repeat: no-repeat;
}

.position-center {
  background-position: center;
}

.cover {
  background-size: cover;
}

.activeYellow {
  color: yellow;
}

.uppercase {
  text-transform: uppercase;
}

/* color */

.red {
  color: red;
}

.youtube-red {
  color: #bb0000;
}

.red-white {
  color: #ff7676;
}

.blue {
  color: #08c;
}

.green {
  color: green;
}

.yellow {
  color: yellow;
}

.yellow-green {
  color: yellowgreen;
}

.forest-green {
  color: forestgreen;
}

.black {
  color: #121212;
}

.white {
  color: #efefef;
}

.border-white {
  border: 1px solid rgba(238, 238, 238, 1);
}

/* no series */

.no-text-decoration {
  text-decoration: none;
}

.no-list-style {
  list-style: none;
  list-style-type: none;
}

.x-outline {
  outline: none;
}

.x-overflow {
  overflow: hidden;
}

.x-border-style {
  border-style: none;
}

.x-margin {
  margin: 0;
}

.x-padding {
  padding: 0;
}

/* effects */

.opacity {
  opacity: 0.8;
}

.more-opacity {
  opacity: 0.5;
}

.opacity--a-point {
  opacity: 0.1
}

.hover:hover {
  opacity: 0.8;
}

.hover-more-opacity:hover {
  opacity: 0.5;
}

.hover-theme-black:hover {
  background-color: #121212;
}

.hover-blue:hover {
  color: #08c;
}

.hover-red:hover {
  color: red;
}

.hover-black:hover {
  color: black;
}

.hover-white-black:hover {
  color: #efefef;
}

.hover-white:hover {
  color: white;
}

.hover-red-white:hover {
  color: #ff7676;
}

.hover-green:hover {
  color: green;
}

.cursor-pointer {
  cursor: pointer;
}

.text-shadow-yellow:hover {
  text-shadow: 0 0 10px yellow;
}

.link--active-blue {
  text-shadow: 0 0 0.5rem #08c;
}

.link--active-red {
  text-shadow: 0 0 0.5rem red;
}

.link--active-yellow {
  text-shadow: 0 0 10px yellow;
}

.thumbnail {
  transition: transform 1s ease;
  :hover {
    opacity: 0.8;
    box-shadow: 0 0 0.5rem rgba(0, 130, 176, 0.8);
    cursor: pointer;
  }
}

.box-shadow-black {
  box-shadow: 0 0 0.5rem 0.1rem #121212;
}

.box-shadow-white {
  box-shadow: 0 0 0.5rem 0.1rem white;
}

.box-shadow-white-black {
  box-shadow: 0 0 0.5rem 0.1rem #efefef;
}

.box-shadow-red {
  box-shadow: 0 0 0.5rem 0.1rem red;
}

.box-shadow-green {
  box-shadow: 0 0 0.5rem 0.1rem forestgreen;
}

.box-shadow-blue {
  box-shadow: 0 0 0.5rem 0.1rem #08c;
}

.box-shadow-white-red {
  box-shadow: 0 0 0.5rem 0.1rem #ff7676;
}

.in-box-shadow-black {
  box-shadow: inset 0 0 0.2rem 0.1rem #121212;
}

.in-box-shadow-blue {
  box-shadow: inset 0 0 0.2rem 0.1rem #08c;
}

/* position */

.fixed {
  position: fixed;
}

.absolute {
  position: absolute;
}

.sticky {
  position: sticky;
}

.relative {
  position: relative;
}

.index-one {
  z-index: 1;
}

.index-ten {
  z-index: 10;
}

/* layout */

.main-width {
  width: 66rem;
}

.side-width {
  width: 29.2rem;
}

.text-center {
  text-align: center;
}

.border {
  border-radius: 0.2rem;
}

.border-round {
  border-radius: 50%;
}

.hover-round:hover {
  border-radius: 50%;
}

.width-vw {
  width: 100vw;
}

.width-percent {
  width: 100%;
}

.height-percent {
  height: 100%;
}

.height-vh {
  height: 100vh;
}

.max-width {
  max-width: 100%;
}

.flex {
  display: flex;
}

.left-auto {
  margin-left: auto;
}

.right-auto {
  margin-right: auto;
}

.center-auto {
  margin: auto;
}

.center-auto-margin {
  margin: 0 auto;
}

.center {
  display: flex;
  justify-content: center;
  align-items: center;
}

.center-percent-absolute {
  position: absolute;
  margin: 0;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.center-percent-relative {
  position: relative;
  margin: 0;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.column-center {
  display: flex;
  flex-flow: column wrap;
  justify-content: center;
  align-items: center;
}

.inline {
  display: inline;
}

.background-cover {
  background-size: cover;
  background-repeat: no-repeat;
  background-position: center;
}

.background-percent {
  background-position: center;
  background-repeat: no-repeat;
  background-size: 100% 100%;
}

.nav-height {
  height: 5.6rem;
}

.padding-for-nav {
  padding-top: 5.6rem;
}

.activeYellow {
  color: yellow;
}

.box-shadow-menu {
  position: relative;
  padding-left: 1.25em;
}

/* scroll */

.oveflow-x-scroll {
  overflow-x: scroll;
}

/* To fix it to the top, use it with fixed, use it for share at mobile mode */

.top {
  top: 0;
  left: 0;
  bottom: 0;
  right: 0;
}

/* To fix it to the bottom, use it with fixed, use it for share at mobile mode */

.sub {
  left: 0;
  bottom: 0;
  right: 0;
}

.sub-navbar--about--s {
  display: none;
}

.sub-navbar--share {
  display: none;
}

/* blog */

.content-break {
  /* margin-top: 2rem; */
  margin-top: 1rem;
  margin-left: 0.2rem;
  margin-right: 0.2rem;
  opacity: 0.7rem;
}

.scroll-top {
  margin-left: auto;
  text-transform: capitalize;
  cursor: pointer;
  margin-top: 0.5rem;
  display: none;
  /* remove two lines below if you don't like this */
  position: fixed;
  right: 2rem;
  bottom: 2rem;
  /* bottom: 6.5rem; */
  /*  */
  font-size: 2rem;
  :hover {
    opacity: 0.8;
    color: red;
  }
}

/* Side */

.aside {
  max-width: 29.2rem;
  /* position: absolute;
  top: 4rem;
  right: 12rem; */
  /* right: 28rem; */
  /* width: 29.2rem; */
}

.stager-wrapper {
  width: 3.6px;
  height: 3.6px;
  position: absolute;
  z-index: 3;
}

.stager {
  border-radius: 50%;
  background-color: white;
  width: 5rem;
  height: 5rem;
  border: 0.5rem solid white;
  position: absolute;
  background-size: 10rem;
  background-repeat: none;
  background-position: center;
}

/* filter */

.filter-grayscale {
  filter: grayscale(100%);
}

.filter-contrast {
  filter: contrast(250%);
}

/* Steadylearner */

.steadylearner-brand--s {
  width: 2.5rem;
  height: 2.5rem;
  border-radius: 0.2rem;
}

/* programming languages */

.programming-language--javacript {
  padding-left: 0.15rem;
  padding-right: 0.15rem;
}

/* media query */

.appear-at-mobile {
  display: none;
}

.appear-at-mobile--flex {
  display: none;
}

.disapper-at-mobile {
  display: inheirt;
}

@media all and (max-width: 75.6rem) {
  .content__main-wrapper {
    width: 100vw;
  }
  .content__main {
    margin: 0 auto 1rem;
    /* margin: 0 auto; */
    padding: 0 1rem;
  }
  .content__side {
    section {
      display: none;
      margin-right: 0;
    }
  }
  /* for containers, You may write various features here.*/
  /* padding-bottom: 5.6rem; */
  /* padding-bottom: 3.7rem; */
  /* .for-sub-navbar--share {
    padding-bottom: 3.7rem;
  } */
  .margin-for-sub-navbar {
    margin-bottom: 5.6rem
  }
  .sub-navbar--about {
    display: none;
  }
  .sub-navbar--about--s {
    display: flex;
  }
  .scroll-top {
    opacity: 0.5;
    font-size: 2.5rem;
  }
  /* You don't need print feature other than computers */
  .print {
    display: none;
  }
  .appear-at-mobile {
    display: inherit;
  }
  .appear-at-mobile--flex {
    display: flex;
  }
  .disappear-at-mobile {
    display: none;
  }
  .no-background-at-mobile {
    background: none;
  }
}

.mobile-share__components {
  display: block;
  text-align: center;
  transition: all 0.6s ease;
  color: white;
  /* margin-right: 0.01rem; */
}

.disappear-at-mobile--slowly {
  display: inherit;
}

.appear-at-mobile--slowly {
  display: block;
}

@media all and (max-width: 66rem) {
  .content__main {
    width: inherit;
  }
}

/* When device size is closer to main cotent, remove side contents */

@media all and (max-width: 48rem) {
  background: none;
  .theme--black-white-crossed-line,
  .theme--black-white-relevant,
  .theme--pink-pattern,
  .theme--green-crossed-lines,
  .theme--blue-square,
  .theme--blue-tile,
  .theme--blue-burn {
    background: none;
  }
  .sub-navbar--about--s {
    display: none;
  }
  .sub-navbar--share {
    display: flex;
  }
  /* .display-none-at-480px {
    display: none
  }

  .display-flex-at-480px {
    display: flex;
  } */
  .YouTube-heart-image {
    width: 4.8rem;
    /* margin-bottom: 0.5rem; */
  }

  .disappear-at-mobile--slowly {
    display: none;
  }

  .appear-at-mobile--slowly {
    display: inherit;
  }
}

${animated}


`;

export default SteadylearnerCSS;

// Loader, StyledLink
