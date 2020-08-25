// Refer to https://github.com

import React from "react";
import { key } from "prop-passer";

import {
    TopNavbar as CSS,
    topnavLinkWrapperClass,
    logoClass,
    topnavLinkClass,
} from "./CSS/TopNavbar";
import { rel, StyledNavLink } from "../link";

export default function TopNavbar() {
    return (
        <CSS>
            <header className="fixed border-white index-one" >
                <nav>
                    <ul
                        className={topnavLinkWrapperClass}
                        role="navigation"
                    >
                        <li
                            className={topnavLinkClass}
                            key={key()}
                        >
                            <Logo />
                        </li>
                        <li
                            className={topnavLinkClass}
                            key={key()}
                        >
                            <Home />
                        </li>
                        <li
                            className={topnavLinkClass}
                            key={key()}
                        >
                            <Undefined />
                        </li>
                        <li
                            className={topnavLinkClass.concat("left-auto")}
                            key={key()}
                        >
                            <SignIn />
                        </li>
                        <li
                            className={topnavLinkClass.concat("topnav__list__last")}
                            key={key()}
                        >
                            <SignUp />
                        </li>
                    </ul>
                </nav>
            </header>
        </CSS>
    );
}

const Logo = () => {
    return (<a
        href="https://www.steadylearner.com"
        className={logoClass}
        target="_blank"
        rel={rel}
        title="This is a link to the website of the author."
        key={key()}
    >
        <span className="flex center" >
            © Steadylearner
	</span>
    </a>);
};

const Home = () => {
    return (<StyledNavLink exact to='/' activeClassName="link--active" >
        <span title="This is a link to /." >
            <i className="fas fa-home" /> Home
        </span>
    </StyledNavLink>);
};

const SignIn = () => {
    return (<StyledNavLink exact to='/login' activeClassName="link--active" >
        <span
            // className="link__box--white"
            title="This is a link to Sign Up page."
        >
            Sign In
	</span>
    </StyledNavLink>);
};

const SignUp = () => {
    return (<StyledNavLink exact to='/register' activeClassName="link--active" >
        <span
            className="link__box--white"
            title="This is a link to Sign Up page."
        >
            Sign Up
	</span>
    </StyledNavLink>);
};

// This is to test undefined paths.
const Undefined = () => {
    return (<StyledNavLink exact to='/undefined' activeClassName="link--active" >
        <span>
            Undefined
        </span>
    </StyledNavLink>);
};
