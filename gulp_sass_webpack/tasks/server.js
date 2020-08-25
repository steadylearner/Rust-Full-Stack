import gulp    from 'gulp'
import Browser from 'browser-sync'
import webpack from 'webpack'
import webpackDevMiddleware from 'webpack-dev-middleware'
import { css } from "./css";

import { config as webpackConfig } from './webpack'

const browser = Browser.create()
const bundler = webpack(webpackConfig)

const reloadDevServer = () => {
    gulp.watch('site/index.html').on('change', () => browser.reload());
    gulp.watch('site/scss/*.scss').on('change', () => {
        css();
        browser.reload();
    });
    gulp.watch('site/js/*.js').on('change', () => browser.reload());
}

export function development() {

    let config = {
        server: 'site',
        open: false,
        middleware: [
            webpackDevMiddleware(bundler, { /* options */ })
        ],
    }

    browser.init(config)

    // https://www.npmjs.com/package/gulp-watch
    reloadDevServer();
}
