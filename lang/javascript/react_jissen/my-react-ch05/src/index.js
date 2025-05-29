import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import LazyBasic from './LazyBasic';
import LazyMulti from './LazyMulti';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <>
    <h1>xxx</h1>
    <h1>xxx</h1>
    <h1>xxx</h1>
    <h1>xxx</h1>
    <h1>xxx</h1>
    <h1>xxx</h1>
    <h1>xxx</h1>

    <h1>LazyMulti</h1>
    <LazyMulti />

    <h1>LazyBasic</h1>
    <LazyBasic />
  </>
);
