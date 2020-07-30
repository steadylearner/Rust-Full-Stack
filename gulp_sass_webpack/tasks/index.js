// equal to gulpfile.js, https://gulpjs.com/docs/en/getting-started/quick-start
import gulp from 'gulp';

import { clean } from "./clean";
import { css } from "./css";
import { development }  from './server'
import { production } from './webpack'

// https://gulpjs.com/docs/en/api/series, https://gulpjs.com/docs/en/getting-started/creating-tasks
// https://gulpjs.com/docs/en/getting-started/async-completion#no-synchronous-tasks
const dev = gulp.series(clean, css, development )
export const build = gulp.series( clean, css, production )

export default dev
