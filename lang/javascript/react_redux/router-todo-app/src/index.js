import React from 'react';
import ReactDOM from "react-dom/client";
import { Route } from 'react-router-dom';
import { Provider } from 'react-redux';
import { ConnectedRouter } from 'react-router-redux';
import { createBrowserHistory } from 'history';
import TodoApp from './containers/TodoApp';
import Error from './components/Error';
import createStore from './store';

const history = createBrowserHistory();
const store = createStore(history);

ReactDOM.createRoot(document.getElementById('root')).render(
    <Provider store={store}>
        <ConnectedRouter history={history}>
            <Route exec path="/" component={TodoApp} />
            <Route exec path="/error" component={Error} />
        </ConnectedRouter>
    </Provider>
)
