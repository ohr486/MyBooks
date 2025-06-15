import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import { RouterProvider } from 'react-router-dom';
import { HelmetProvider } from 'react-helmet-async';
import routesBasic from './routesBasic';
import routesLink from './routesLink';
import routesLink2 from './routesLink2';
import routesLink3 from './routesLink3';
import routesLink4 from './routesLink4';
import routesParam from './routesParam';
import routesHandle from './routesHandle';

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

        <h1>routerHandle/RouterParam2</h1>
        <HelmetProvider>
            <RouterProvider router={routesHandle} />
        </HelmetProvider>
        <SepLine />

        <h1>routesParam</h1>
        <RouterProvider router={routesParam} />
        <SepLine />

        <h1>routesLink4/RouterApp2</h1>
        <RouterProvider router={routesLink4} />
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
