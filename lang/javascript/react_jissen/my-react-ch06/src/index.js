import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import MaterialBasic from './MaterialBasic';
import MaterialDrawer from './MaterialDrawer';
import MaterialGrid from './MaterialGrid';
import MaterialTheme from './MaterialTheme';
import MaterialMode from './MaterialMode';
import FormMui from './FormMui';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <>
    <h1>xxx</h1>
    <h1>xxx</h1>
    <h1>xxx</h1>
    <h1>FormMui</h1>
    <FormMui />

    <h1>MaterialMode</h1>
    <MaterialMode />

    <h1>MaterialTheme</h1>
    <MaterialTheme />

    <h1>MaterialGrid</h1>
    <MaterialGrid />

    <h1>MaterialDrawer</h1>
    <MaterialDrawer />

    <h1>MaterialBasic</h1>
    <MaterialBasic />
  </>
);
