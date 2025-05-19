import React from 'react';
import ReactDOMServer from 'react-dom/server';

const html = ReactDOMServer.renderToStaticMarkup(
    <h1>Hello, SSR!</h1>
);

console.log(html);