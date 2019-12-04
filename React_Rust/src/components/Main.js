import React, { lazy, Suspense } from "react";
import {
	Switch,
	Route,
} from "react-router-dom";


import Loader from "./reusable/Loader";

import User from "./User";
import Home from "./Home";
// Is this the reason for chunk1 file not found? or webpack or other dependencies?
// const Home = lazy(() => import("./Home")); // Make Login with Navbar
// const User = lazy(() => import("./User"));

export default function Main() {
	return (
		<Suspense
			// Remove duplicate maxduation
			fallback={
				<Loader>
					<i className="circle-spinner--s fixed border-round" />
					<i
						className="
							fab fa-youtube
							youtube-spinner--s
							fixed
							red
						"
					/>
				</Loader>
			}
		>
			<Switch>
				<Route exact path="/">
					<Home />
				</Route>
				<Route path="/user">
					<User />
				</Route>
			</Switch>
		</Suspense>
	);
}
