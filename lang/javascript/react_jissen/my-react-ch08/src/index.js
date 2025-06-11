import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import { RouterProvider } from 'react-router-dom';
import routesBasic from './routesBasic';
import routesLink from './routesLink';
import routesLink2 from './routesLink2';
import routesLink3 from './routesLink3';

const SepLine = () => (
    <hr
        style={{
            color: "black",
            backgroundColor: "black",
            height: 2
        }} />
);

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
    <>
        <h1>xxx</h1>
        <SepLine />

        <h1>xxx</h1>
        <SepLine />

        <h1>xxx</h1>
        <SepLine />

        <h1>xxx</h1>
        <SepLine />

        <h1>xxx</h1>
        <SepLine />

        <h1>routesLink3/RouterNav2</h1>
        <RouterProvider router={routesLink3} />
        <SepLine />

        <h1>routesLink2/RouterNav</h1>
        <RouterProvider router={routesLink2} />
        <SepLine />

        <h1>routesLink/RouterApp</h1>
        <RouterProvider router={routesLink} />
        <SepLine />

        <h1>routesBasic</h1>
        <RouterProvider router={routesBasic} />
        <SepLine />
    </>
);
