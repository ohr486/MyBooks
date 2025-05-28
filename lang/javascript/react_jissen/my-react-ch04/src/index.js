import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import reportWebVitals from './reportWebVitals';
import StateForm from './StateForm';
import StateFormUC from './StateFormUC';
import FormTextarea from './FormTextarea';
import FormSelect from './FormSelect';
import FormList from './FormList';
import FormRadio from './FormRadio';
import FormCheck from './FormCheck';
import FormCheckMulti from './FormCheckMulti';
import FormFile from './FormFile';
import StateNest from './StateNest';
import StateNestImmer from './StateNestImmer';
import StateNestImmer2 from './StateNestImmer2';
import StateTodo from './StateTodo';
import FormBasic from './FormBasic';
import FormYup from './FormYup';
import FormJapan from './FormJapan';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
    <>
        <h1>FormJapan</h1>
        <FormJapan />

        <h1>FormYup</h1>
        <FormYup />

        <h1>FormBasic</h1>
        <FormBasic />

        <h1>StateTodo</h1>
        <StateTodo />

        <h1>StateNestImmer2</h1>
        <StateNestImmer2 />

        <h1>StateNestImmer</h1>
        <StateNestImmer />

        <h1>StateNest</h1>
        <StateNest />

        <h1>FormFile</h1>
        <FormFile />

        <h1>FormCheckMulti</h1>
        <FormCheckMulti />

        <h1>FormCheck</h1>
        <FormCheck />

        <h1>FormRadio</h1>
        <FormRadio />

        <h1>FormList</h1>
        <FormList />

        <h1>FormSelect</h1>
        <FormSelect />

        <h1>FormTextarea</h1>
        <FormTextarea />

        <h1>StateFormUC</h1>
        <StateFormUC />

        <h1>StateForm</h1>
        <StateForm />
    </>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
