import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';


const serverNowString = document.getElementById('server-now').getAttribute('data-server-now');

const now = new Date(
    parseInt(serverNowString, 10)
);

// React15以前
//ReactDOM.createRoot(document.getElementById('root')).render(
//    <App renderedAt={now} />
//);

// React16以降
ReactDOM.createRoot(document.getElementById('root')).hydrate(
    <App renderedAt={now} />
);
