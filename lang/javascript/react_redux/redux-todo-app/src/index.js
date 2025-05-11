import React from 'react';
import { Provider } from 'react-redux';
import { createStore } from 'redux';
import ReactDOM from "react-dom/client";
import taskReducer from "./reducers/tasks";
import TodoApp from "./containers/TodoApp";

const store = createStore(taskReducer);

ReactDOM.createRoot(document.getElementById('root')).render(
    <Provider store={store}>
        <TodoApp />
    </Provider>
)
