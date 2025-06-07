import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import StateEffect from './StateEffect';
import HookTimer from './HookTimer';
import HookEffect from './HookEffect';
import HookRefNg from './HookRefNg';
import HookRef from './HookRef';
import HookRefForward from './HookRefForward';
import HookCallbackRef from './HookCallbackRef';
import HookReducer from './HookReducer';
import HookReducerUp from './HookReducerUp';
import HookReducerInit from './HookReducerInit';
import HookContext from './HookContext';
import MyThemeProvider from './MyThemeProvider';
import HookThemeButton from './HookThemeButton';
import { RecoilRoot } from 'recoil';
import RecoilCounter from './RecoilCounter';
import RecoilTodo from './RecoilTodo';
import RecoilTodoUp from './RecoilTodoUp';
import HookMemo from './HookMemo';
import HookTransition from './HookTransition';
import HookDeferred from './HookDeferred';
import HookDeferredTransition from './HookDeferredTransition';
import HookCustom from './HookCustom';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
    <>
        <h1>HookCustom</h1>
        <HookCustom />

        <h1>HookDeferredTransition</h1>
        <HookDeferredTransition />

        <h1>HookDeferred</h1>
        <HookDeferred />

        <h1>HookTransition</h1>
        <HookTransition />

        <h1>HookMemo</h1>
        <HookMemo />

        <h1>RecoilTodoUp</h1>
        <RecoilRoot>
            <RecoilTodoUp />
        </RecoilRoot>

        <h1>RecoilTodo</h1>
        <RecoilRoot>
            <RecoilTodo />
        </RecoilRoot>

        <h1>RecoilCounter</h1>
        <RecoilRoot>
            <RecoilCounter />
        </RecoilRoot>

        <h1>MyThemeProvider</h1>
        <MyThemeProvider>
            <HookThemeButton />
        </MyThemeProvider>

        <h1>HookContext</h1>
        <HookContext />

        <h1>HookReducerInit</h1>
        <HookReducerInit init={0} />

        <h1>HookReducerUp</h1>
        <HookReducerUp init={0} />

        <h1>HookReducer</h1>
        <HookReducer init={0} />

        <h1>HookCallbackRef</h1>
        <HookCallbackRef />

        <h1>HookRefForward</h1>
        <HookRefForward />

        <h1>HookRef</h1>
        <HookRef />

        <h1>HookRefNg</h1>
        <HookRefNg />

        <h1>HookEffect</h1>
        <HookEffect init={10} />

        <h1>HookTimer</h1>
        <HookTimer init={10} />

        <h1>StateEffect</h1>
        <StateEffect init={0} />
    </>
);
