import React from "react";
import {
	BrowserRouter as Router,
} from "react-router-dom";

import { TopNav } from "./UI";
import Main from "./Main";

import CustomCSS from "./CSS";

export default function Root() {
	return (
		<Router>
			<CustomCSS>
				<TopNav />
				<Main />
			</CustomCSS >
		</Router>
	);
}
