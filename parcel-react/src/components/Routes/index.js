// Routes/index to modulize it later with this.
// https://reacttraining.com/react-router/web/example/route-config
// Then, it should be routes/ ?

import React from "react";
import {
    Switch,
    Route,
} from "react-router-dom";

import Home from "../views/Home";
import { Register, Login, Profile } from "../views/User";
import { NotFound } from "../views/errors";

import PrivateRoute from "./PrivateRoute";
import { TestPrivatePage } from "../views/private";

export default function Routes() {
    return (
        <Switch>
            <Route exact path="/">
                <Home />
            </Route>
            <Route exact path="/register">
                <Register />
            </Route>
            <Route exact path="/login">
                <Login />
            </Route>
            <PrivateRoute exact path="/authorized" >
                <TestPrivatePage />
            </PrivateRoute>
            <PrivateRoute exact path="/profile" >
                <Profile />
            </PrivateRoute>
            <Route path="*" >
                <NotFound />
            </Route>
        </Switch>
    );
}

