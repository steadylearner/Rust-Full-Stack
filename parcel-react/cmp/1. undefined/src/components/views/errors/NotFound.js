import React from "react";
import {
    useLocation
} from "react-router-dom";

export default function NotFound() {
    const { pathname } = useLocation();

    return (
        <div>
            No contents for {pathname}.
        </div>
    );
}

// How to handle undefined pages.

// 1. Server - You can manually control location with this.
// document.location = "http://localhost:1234/undefined"
// (You should hanlde it at your server.)

// 2. Web(React) - Make component similar to this without components for it
// If you want to test it inside your React app.
// <Link to="/will-not-match">Will Not Match</Link>
// (Make components simialr to this and pair for that NotFound here.)