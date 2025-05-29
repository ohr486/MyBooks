import ReactDOM from 'react-dom/client';
import './index.css';
import LazyBasic from './LazyBasic';
import LazyMulti from './LazyMulti';
import SuspenseSimple from './SuspenseSimple';
import SuspenseResult from './SuspenseResult';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <>
    <h1>xxx</h1>
    <h1>xxx</h1>
    <h1>xxx</h1>
    <h1>xxx</h1>
    <h1>xxx</h1>

    <h1>SuspenseResult</h1>
    <SuspenseResult />

    <h1>SuspenseSimple</h1>
    <SuspenseSimple />

    <h1>LazyMulti</h1>
    <LazyMulti />

    <h1>LazyBasic</h1>
    <LazyBasic />
  </>
);
