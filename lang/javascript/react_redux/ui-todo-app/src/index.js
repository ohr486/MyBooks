import React from 'react';
import { Provider } from 'react-redux';
import { createStore, applyMiddleware } from 'redux';
import ReactDOM from "react-dom/client";
import taskReducer from "./reducers/tasks";
import TodoApp from "./containers/TodoApp";
//import { createLogger } from 'redux-logger';

//const loggerSetting = {
//    predicate: (getState, action) => action.type !== 'HIGH_FREQUENCY_ACTION'
//};
//const logger = createLogger(loggerSetting);

const middleware = store => next => action => {
    const result = next(action);
    return result;
};

const logger = store => next => action => {
    console.log(store.getState());
    console.log(action);
    const result = next(action);

    console.log(store.getState());
    console.log('===========');

    return result;
}

const store = createStore(
    taskReducer,
    applyMiddleware(logger)
);

ReactDOM.createRoot(document.getElementById('root')).render(
    <Provider store={store}>
        <TodoApp />
    </Provider>
)
