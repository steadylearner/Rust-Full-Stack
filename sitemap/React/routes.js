<Switch>
    <Route exact path="/" />
    <Route exact path="/about/:language?" />

    <Route exact path={"/video"} />
    <Route path={"/video/search/:query?"} />
    <Redirect from='/vr/:videoId' />
    <Route path={"/video/watch/:videoId"} />
    <Redirect from='/vw/:videoId' />
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
</Switch>