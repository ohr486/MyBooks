import React from 'react';
import ReactDOM from "react-dom/client";
import taskReducer from "./reducers/tasks";
import TodoApp from "./components/TodoApp";
import { createStore } from 'redux';

const store = createStore(taskReducer);

const root = ReactDOM.createRoot(document.getElementById('root'));

function renderApp(store) {
    root.render(
        <React.StrictMode>
            <TodoApp store={store}/>
        </React.StrictMode>
    )
}

store.subscribe(() => renderApp(store));
renderApp(store);
