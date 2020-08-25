import gulp from "gulp";
import sass from "gulp-sass";
import sassCompiler from "node-sass";

sass.compiler = sassCompiler;

// https://gulpjs.com/docs/en/getting-started/working-with-files
export const css = () => {
    return gulp.src('site/scss/*.scss')
        .pipe(sass({ outputStyle: 'compressed' }).on('error', sass.logError))
        .pipe(gulp.dest('site/css'));
}

// http://zetcode.com/gulp/sass/
// https://www.npmjs.com/package/gulp-sass
