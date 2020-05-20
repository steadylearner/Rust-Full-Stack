import React from "react";
import { Switch, Route } from "react-router";

export default (
    <Switch>
        <Route exact path="/" />
        <Route exact path="/about" />
        <Route path="/about/:language?" />

        <Route exact path={"/video"} />
        <Route path={"/video/search/:query?"} />
        
        <Route path={"/video/watch/:videoId"} />
       
        <Route path={"/video/write/:videoId?"} />

        <Route exact path={"/image"} />
        <Route path={"/image/search/:query?"} />

        <Route exact path={"/blog"} />
        <Route path={"/blog/search/:query?"} />
        <Route path={"/blog/read/:blogTitle?"} />

        <Route exact path="/code" />
        <Route path={"/code/search/:query?"} />

        <Route exact path="/markdown" />
        <Route exact path="/jsx"/>
        <Route exact path={"/slideshow"} />

        <Route path={"/static/images/:folder"} />
    </Switch>
);
