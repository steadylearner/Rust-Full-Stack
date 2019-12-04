import React from "react";
import {
	// Passers,
	key,
} from "prop-passer";

import { TopNavCSS } from "./CSS";
import { StyledNavLink } from "../reusable/StyledLink";

export default function TopNav() {
	const topnavLinkClass = "topnav__list cursor-pointer hover transition-half ";

	return (
		<TopNavCSS>
			<header className="fixed border-white index-one" >
				<nav>
					<ul
						className={`
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
						`}
						role="navigation"
					>
						<li
							className={topnavLinkClass}
							key={key()}
						>
							<a
								href="https://www.steadylearner.com"
								className={`
                                    no-text-decoration
                                    white
                                    flex
                                    center
                                    font-normal
                                    hover
                                    transition-half
								`}
								target="_blank"
								rel="noopener noreferrer"
								title="This is a path to Steadylearner Website"
								key={key()}
							>
								<span className="flex center" >
                                    © Steadylearner
								</span>
							</a>
						</li>
						<li
							className={topnavLinkClass}
							key={key()}
						>
							<StyledNavLink exact to='/user' activeClassName="link--active" >
								<span title="This is a link to /user." >
									<i className="fas fa-user" /> User
								</span>
							</StyledNavLink>
						</li>
						<li
							className={
								topnavLinkClass
									.concat("left-auto topnav__list__last")
							}
							key={key()}
						>
							<StyledNavLink exact to='/' activeClassName="link--active" >
								<span
									className="link__box--white"
									title="This is a link to Sign Up page."
								>
                                    Sign Up
								</span>
							</StyledNavLink>
						</li>
					</ul>
				</nav>
			</header>
		</TopNavCSS>
	);
}
