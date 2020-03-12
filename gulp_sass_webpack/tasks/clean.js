const del = require('del');

export const clean = () => {
    return del([
        'site/css/main.css',
        // 'site/js/bundle.js',
    ]);
}