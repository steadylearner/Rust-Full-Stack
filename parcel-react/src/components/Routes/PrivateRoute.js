import React from "react";
import {
    Route,
    Redirect,
} from "react-router-dom";

import { fakeAuth } from "../auth";

// Wrapper for <Route> that redirects to the login
// redirect if you're not yet authenticated.
export default function PrivateRoute({ children, ...rest }) {
    return (
        <Route
            {...rest}
            render={({ location }) =>
                fakeAuth.isAuthenticated ? (
                    children
                ) : (
                        <Redirect
                            to={{
                                pathname: "/login",
                                state: { from: location }
                            }}
                        />
                    )
            }
        />
    );
}

// (Substitute fakeAuth.isAuthenticated with your own implementation later.)