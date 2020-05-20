import React from "react";
import {
    useHistory,
    useLocation
} from "react-router-dom";

import { fakeAuth } from "../../auth";

function Login() {
    const history = useHistory();
    const location = useLocation();

    // How this work exactly? Why it sends to /authorized?
    // Should read all relevant code.
    // I need to find correctly how all this auth flow work.
    const { from } = location.state || { from: { pathname: "/" } };

    const login = () => {
        fakeAuth.authenticate(() => {
            history.replace(from);
        });
    };

    return (
        <div>
            <p>You must log in to view the page at {from.pathname}</p>
            <button onClick={login}>Log in</button>
        </div>
    );
}

export default Login;