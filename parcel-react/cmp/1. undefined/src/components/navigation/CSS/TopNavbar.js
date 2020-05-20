// eslint-disable-next-line no-unused-vars
import React from "react";
import styled from "styled-components";

const TopNavbar = styled.section`

    .topnav {
        margin: 0;

        display: flex;
        flex: 1 1;
        justify-content: flex-start;
        align-items: center;

        color: white;
        font-size: 1.6rem;
        position: fixed;
        z-index: 1;

        outline: 0.2rem white solid;
    }

    .topnav__list {
        height: 5.6rem;
        line-height: 5.6rem;
        margin-right: 1.6rem;
    }

    .link--active {
        text-shadow: 0 0 1rem yellow;
    }

    .link__box--white {
        border: 0.1rem solid white;
        border-radius: 0.2rem;
        padding: 0.5rem;
    }

    .topnav__list__last {
        margin-right: 6.5rem;
    }

    /* @media all and (max-width: 67.2rem) {
        li:not(:first-child):not(:nth-child(2)):not(:last-child) {
            display: none;
        }
    }

    @media all and (max-width: 48rem) {
        .topnav__list__last {
            margin-right: 7rem;
        }
    }

    @media all and (max-width: 38.4rem) {
        li:not(:nth-child(2)):not(:last-child) {
            display: none;
        }
        /* li:not(:first-child):not(:last-child):not(:nth-child(2)) {
            display: none;
        } */
    } */
`;

const topnavLinkWrapperClass = `
    topnav
    border
    max-width
    no-list-style
    theme-black
    x-overflow
    text-center
    no-text-decoration
    transition
    width-vw
`;

const logoClass = `
    no-text-decoration
    white
    flex
    center
    font-normal
    hover
    transition-half
    bold
`;

const topnavLinkClass = "topnav__list cursor-pointer hover transition-half ";

export {
    TopNavbar,
    topnavLinkWrapperClass,
    logoClass,
    topnavLinkClass,
};