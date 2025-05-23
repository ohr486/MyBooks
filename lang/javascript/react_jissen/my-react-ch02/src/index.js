import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';

//import './class.css';

const root = ReactDOM.createRoot(document.getElementById('root'));

const title = 'これからはじめるVue.js 3実践入門';
root.render(
    <div className="main">
        <p>「{title}」（SBクリエイティブ刊）</p>
        <img src="https://wings.msn.to/books/978-4-8156-1336-5/978-4-8156-1336-5.jpg"
             alt={title} />
        絶賛発売中
    </div>
);

//root.render(
//    <p className="hoge">WINGSプロジェクト</p>
//);

//const props = 'color: White; background-color: Blue; padding: 3px';
//const props = {
//    color: 'White',
//    backgroundColor: 'Blue',
//    padding: 3
//};
//root.render(
//    <p style={props}>WINGSプロジェクト</p>
//);

//const attrs = {
//    href: 'https://ja.react.dev',
//    download: false,
//    target: '_blank',
//    rel: 'help'
//};
//root.render(
//    <>
//        <a href={attrs.href} download={attrs.download}
//           target={attrs.target} rel={attrs.rel}>サポートページ1</a>
//        <a {...attrs}>サポートページ2</a>
//    </>
//);

//const dest = 'https://ja.react.dev';
//root.render(
//    <>
//        <a href={dest}>React本家サイト</a>
//        <a href={dest + '/docs'}>React本家サイト</a>
//        <a href={`${dest}/docs`}>React本家サイト</a>
//    </>
//);

//root.render(
//    <ul>
//        <li>{true}</li>
//        <li>{false}</li>
//        <li>{undefined}</li>
//        <li>{null}</li>
//        <li>{0}</li>
//        <li>{String(true)}</li>
//    </ul>
//);

//root.render(
//    <>
//        <div>{`Tom &amp; Jerry`}</div>
//        <div>Tom &amp; Jerry</div>
//        <div>{'Tom ¥u0026 Jerry'}</div>
//        <div>{`Tom ${String.fromCodePoint(38)} Jerry`}</div>
//        <div dangerouslySetInnerHTML={{__html: 'Tom &amp; Jerry'}}></div>
//    </>
//);

//const content = `<h3>WINGSプロジェクト</h3>
//  <img src="https://wings.msn.to/image/wings.jpg" />`;
//root.render(<p dangerouslySetInnerHTML={{__html: content}}></p>);

//setInterval(() => {
//    root.render(
//        <p>現在時刻: {(new Date()).toLocaleString()}</p>
//    );
//}, 1000);

//const name = '鈴木';
//root.render(
//    <>
//        <p>こんにちは、世界！</p>
//        <p>はじめまして、React！</p>
//        <p>こんにちは、{name}です！</p>
//    </>
//);

//root.render(
//  <React.StrictMode>
//    <App />
//  </React.StrictMode>
//);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
//reportWebVitals();
reportWebVitals(console.log);
