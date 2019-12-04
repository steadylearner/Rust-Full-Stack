// eslint-disable-next-line no-unused-vars
import React from "react";
import styled from "styled-components";

const UserCSS = styled.section`
    display: flex;
    margin: 0 auto;
    font-size: 2rem;
    max-width: 64rem;
    width: 64rem;
    padding: 5.6rem 0 0;
    background-color: white;

    .user-list {
        display: flex;
        flex-flow: row nowrap;
        padding: 1rem 2rem 2rem 2rem;
    }

    .user-list__main {
        flex: 6 1 0%
    }

    .user-list__image {
        flex: 2 2 auto;
        margin-top: 1.5rem;
    }

    .user-list__name {
        margin-top: 1.5rem;
        font-size: 2rem;
    }

    .user-list__email {
        opacity: 0.5;
        font-size: 1.6rem;
        font-family: "Times New Roman", Times, serif;
        margin-right: 1rem;
    }

    .user-list__util {
        display: flex;
        flex-flow: row wrap;
        margin-top: 1rem;
    }

    .user-image {
        height: 4.8rem;
        width: 4.8rem;
    }

    @media all and (max-width: 76.8rem) {
        max-width: 100%;
    }

    @media all and (max-width: 48rem) {
        /* .user-list__image {
        
        } */

        .user-list__name {
            /* font-size: 2rem; */
        }

        .user-list__email {
            font-size: 1.6rem;
        }

        /* .user-list__util {

        } */

        /* .user-list__author {
            display: none;
        } */

        .user-list__cpf {
            font-size: 1.6rem;
        }

        .user-list__telefone {
            font-size: 1.6rem;
        }

        .user-list__remove {
            font-size: 1.6rem;
        }

        /* .user-image {

        } */
    }

    @media all and (max-width: 38.4rem) {
        /* .user-list__image {
        
        } */

        .user-list__name {
            font-size: 1.6rem;
        }

        .user-list__email {
            font-size: 1.2rem;
        }

        /* .user-list__util {

        } */

        /* .user-list__author {
            display: none;
        } */

        .user-list__cpf {
            font-size: 1rem;
        }

        .user-list__telefone {
            font-size: 1rem;
        }

        /* .user-image {

        } */
    }
`;

export default UserCSS;
