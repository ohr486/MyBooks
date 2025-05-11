import {
    createStore as reduxCreateStore,
    combineReducers,
    applyMiddleware
} from 'redux';
import { routerReducer, routerMiddleware } from 'react-router-redux';
import taskReducer from '../reducers/tasks';

export default function createStore(history) {
    return reduxCreateStore(
        combineReducers({
            tasks: taskReducer,
            router: routerReducer
        }),
        applyMiddleware(
            routerMiddleware(history)
        )
    );
}