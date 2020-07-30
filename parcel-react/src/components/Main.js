// import React from "react";
// import {
//     BrowserRouter as Router,
//     Switch,
//     Route,
// } from "react-router-dom";

import React from "react";
import {
    BrowserRouter as Router,
} from "react-router-dom";

import { TopNavbar } from "./navigation";
import Routes from "./Routes";

export default function Main() {
    return (
        <Router>
            <div>
                <TopNavbar />
                <main
                    // Temporary layout solution for this example
                    style={{ "padding-top": "6rem", "font-size": "2rem", "color": "#08c", }}
                >
                    <Routes />
                </main>
            </div>
        </Router>
    );
}

