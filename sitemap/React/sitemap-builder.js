require('@babel/register');
 
const router = require('./router').default;
const Sitemap = require('react-router-sitemap').default;
 
(
    new Sitemap(router)
        .build('http://www.steadylearner.com')
        .save('./sitemap.xml')
);

console.log("The sitemap was built.");
