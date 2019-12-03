import React from "react";
import styled from "styled-components";

const NavbarCSS = styled.header.attrs({
    className: "fixed border-white index-one",
})`
    .navbar {
        margin: 0;

        display: flex;
        flex: 1 1;
        justify-content: flex-start;
        align-items: center;

        color: white;
        font-size: 1.6rem;
        position: fixed;
        z-index: 1;

        outline: 0.2px white solid;
    }

    .navbar__brand {
        width: 1.6rem;
        height: 1.6rem;
    }

    .navbar__youtube {
        width: 2rem;
        height: 2rem;
        margin-right: 0.1rem;
    }

    .navbar__list {
        height: 5.6rem;
        line-height: 5.6rem;
        margin-right: 1.6rem;
    }

    .navbar__list__search {
        margin-right: 6rem;
        font-size: 1.7rem;
    }

    .link--active {
        text-shadow: 0 0 1rem yellow;
    }

    .search-container {
        width: 10rem;
    }

    .search-icon {
        font-size: 2rem;
    }

    .navbar__current {
        text-shadow: 0 0 1rem yellow;
    }

    .navbar__list__last {
        margin-right: 7.5rem;
    }

    /* @media all and (max-width: 96rem) {

        li:not(:first-child):not(:nth-child(2)):not(:last-child) {
            display: none;
        }

        .box-shadow-menu:before {
            left: -1rem;
        }

        .navbar__list__search {
            margin-right: 6rem;
        }
    } */

    @media all and (max-width: 67.2rem) {
        li:not(:first-child):not(:nth-child(2)):not(:last-child) {
            display: none;
        }

        .box-shadow-menu:before {
            left: -1rem;
        }

        .navbar__list__search {
            margin-right: 6rem;
        }

        .search-input {
            /* padding: 1.6rem;
            margin-top: 0;
            font-size: 2rem;
            position: absolute;
            left: 0;
            width: calc(100vw - 10rem); */
            
            /* padding: 1.1rem; */
            /* margin: 0.2rem; */
            
            padding: 1.25rem;
            margin: 0.25rem;

            font-size: 2.5rem;
            position: absolute;
            left: 0;
            width: calc(100vw - 8.5rem);
        }
    }

    @media all and (max-width: 48rem) {

    }

    @media all and (max-width: 38.4rem) {
        li:not(:first-child):not(:last-child) {
            display: none;
        }
        /* li:not(:first-child):not(:last-child):not(:nth-child(2)) {
            display: none;
        } */
    }
`;

export default props => <NavbarCSS {...props} />;