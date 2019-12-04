/*eslint-disable */
import React from "react";
/*eslint-disable */
import styled, { keyframes } from "styled-components";

const rotate360 = keyframes`
  0% {
    opacity: 0;
    transform: rotate(0);
  }
  ${"" /* 60% {
    transform: rotate(100deg);
  } */}
  100% {
    opacity: 1;
    transform: rotate(360deg);
    ${""}
  }
`;

// const rRotate360 = keyframes`
//   0% {
//     opacity: 0;
//     transform: rotate(360deg);
//   }
//   ${"" /* 60% {
//     transform: rotate(100deg);
//   } */}
//   100% {
//     opacity: 1;
//     transform: rotate(0);
//     ${""}
//   }
// `;

const Loader = styled.section`

      .youtube-spinner {
        display: inline-block;
        animation: ${rotate360} 1.5s linear infinite 0.5s;
        font-size: 2rem;

        ${""}
        z-index: 5;
        height: 1em;
        width: 1em;
        overflow: show;
        margin: auto;
        top: 0;
        left: 0;
        bottom: 0;
        right: 0;

        &:hover {
          color: black;
        }
      }

      .youtube-spinner--s {
        display: inline-block;
        animation: ${rotate360} 1s linear infinite 0.5s;
        font-size: 1rem;

        z-index: 10;
        height: 1em;
        width: 1em;
        overflow: show;
        margin: auto;
        top: 0;
        left: 0;
        bottom: 0;
        right: 0;

        &:hover {
          color: black;
        }
      }

      .circle-spinner {
        animation: ${rotate360} 1.5s linear infinite 0.5s;

        border-top: 1rem solid #08c;
        border-left: 1rem solid forestgreen;
        border-right: 1rem solid red;
        border-bottom: 1rem solid yellow;

        z-index: 10;
        height: 5rem;
        width: 5rem;
        overflow: show;
        margin: auto;
        top: 0;
        left: 0;
        bottom: 0;
        right: 0;
      }

      .circle-spinner--l {
        animation: ${rotate360} 1.5s linear infinite 0.5s;

        border-top: 2rem solid yellow;
        border-left: 2rem solid forestgreen;
        border-right: 2rem solid red;
        border-bottom: 2rem solid #08c;

        z-index: 10;
        height: 10rem;
        width: 10rem;
        overflow: show;
        margin: auto;
        top: 0;
        left: 0;
        bottom: 0;
        right: 0;
      }

      .circle-spinner--s {
        animation: ${rotate360} 1s linear infinite 0.5s;

        border-top: 0.5rem solid #08c;
        border-left: 0.5rem solid forestgreen;
        border-right: 0.5rem solid red;
        border-bottom: 0.5rem solid yellow;

        z-index: 10;
        height: 2.5rem;
        width: 2.5rem;
        overflow: show;
        margin: auto;
        top: 0;
        left: 0;
        bottom: 0;
        right: 0;
      }
`;

export default Loader;
