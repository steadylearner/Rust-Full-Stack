/*eslint-disable */
import React from "react";
/*eslint-disable */
import { Link, NavLink } from "react-router-dom";
import styled from "styled-components";

const LinkCSS = `
    text-decoration: none;
    outline: none;

    &:focus,
    &:hover,
    &:visited,
    &:link, {
        text-decoration: none;
    }
`;

const StyledLink = styled(Link)`
    ${LinkCSS}
`;

const StyledNavLink = styled(NavLink)`
    ${LinkCSS}

    color: white;

    &:hover {
        opacity: 0.7;
    }

`;

export {
	StyledLink,
	StyledNavLink,
};
