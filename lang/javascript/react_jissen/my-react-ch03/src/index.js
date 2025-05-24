import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import reportWebVitals from './reportWebVitals';
//import App from './App';
//import './MyHello';
import MyHello from "./MyHello";
import EventBasic from "./EventBasic";
import StateBasic from "./stateBasic";
import books from './books';
import ForList from './ForList';
import ForNest from './ForNest';
import ForFilter from './ForFilter';
import ForSort from './ForSort';
import SelectStyle from "./SelectStyle";

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
    <>
        <h1>SelectStyle</h1>
        <SelectStyle mode="light" />
        <SelectStyle mode="dark" />
        <SelectStyle mode="foo" />

        <h1>ForSort</h1>
        <ForSort src={books} />

        <h1>ForFilter</h1>
        <ForFilter src={books} />

        <h1>ForNest</h1>
        <ForNest src={books} />

        <h1>ForList</h1>
        <ForList src={books} />

        <h1>StateBasic</h1>
        <StateBasic init={0} />

        <h1>EventBasic</h1>
        <h2>type=time</h2><EventBasic type="time" />
        <h2>type=date</h2><EventBasic type="date" />
        <h2>type=''</h2><EventBasic />

        <h1>MyHello</h1>
        <MyHello myName="鈴木" />
        <MyHello myName={['山田', '鈴木', '佐藤']} />
        <MyHello myName={() => { console.log('Hoge'); }} />
        <MyHello />
    </>
);

//<MyHello myName={{ name: '鈴木', age: 48 }} />

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
