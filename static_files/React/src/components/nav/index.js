import React from 'react';
import { key } from "prop-passer";

import NavClass from "./CSS";

const Nav = () => {
    const navclass = `
        navbar border max-width no-list-style
        theme-black x-overflow text-center no-text-decoration
        transition width-vw
	`;
	
	const listclass = "navbar__list cursor-pointer hover";

    return (
        <NavClass>
            <nav>
                <ul
                    className={navclass}
                    role="navigation"
                >
                    <li
                        className={listclass}
                        key={key()}
                    >
                        <a
                            href="https://www.steadylearner.com/blog"
                            className="white no-text-decoration font-normal hover transition-half"
                            target="_blank"
                            rel="noopener noreferrer"
                            title="Steadylearner Blog"
                        >
                            ©ode
                        </a>
                    </li>
                    {/* <li
                        className={listclass}
                        key={key()}
                    >
                        About
                    </li> */}
                    <li
                        className={listclass.concat(" left-auto margin-right-six")}
                        key={key()}
                    >
                        <span
                            className="border-white pad-button"
                            title="Clique para começar a usar esse site."
                        >
                            Sign Up
                        </span>
                    </li>
                </ul>
            </nav>
        </NavClass>
    )
}

export default Nav;