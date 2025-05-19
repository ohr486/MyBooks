import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';


const serverNowString = document.getElementById('server-now-string');

const now = new Date(
    parseInt(serverNowString, 10)
);

ReactDOM.createRoot(document.getElementById('root')).render(
    <App renderedAt={now} />
);