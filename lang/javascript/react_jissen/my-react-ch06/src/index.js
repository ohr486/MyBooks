import React, { Suspense } from 'react';
import ReactDOM from 'react-dom/client';
import { ErrorBoundary } from 'react-error-boundary';
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import './index.css';
import MaterialBasic from './MaterialBasic';
import MaterialDrawer from './MaterialDrawer';
import MaterialGrid from './MaterialGrid';
import MaterialTheme from './MaterialTheme';
import MaterialMode from './MaterialMode';
import FormMui from './FormMui';
import QueryPre from './QueryPre';
import QueryBasic from './QueryBasic';
import QuerySuspense from './QuerySuspense';

const cli = new QueryClient();
const cliSusp = new QueryClient({
    defaultOptions: {
        queries: {
            suspense: true
        },
    },
});

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <>
    <h1>QuerySuspense</h1>
    <Suspense fallback={<p>Loading...</p>}>
        <ErrorBoundary fallback={<div>エラーが発生しました。</div>}>
            <QueryClientProvider client={cliSusp}>
                <QuerySuspense />
            </QueryClientProvider>
        </ErrorBoundary>
    </Suspense>

    <h1>QueryBasic</h1>
    <QueryClientProvider client={cli}>
        <QueryBasic />
    </QueryClientProvider>

    <h1>QueryPre</h1>
    <QueryPre />

    <h1>FormMui</h1>
    <FormMui />

    <h1>MaterialMode</h1>
    <MaterialMode />

    <h1>MaterialTheme</h1>
    <MaterialTheme />

    <h1>MaterialGrid</h1>
    <MaterialGrid />

    <h1>MaterialDrawer</h1>
    <MaterialDrawer />

    <h1>MaterialBasic</h1>
    <MaterialBasic />
  </>
);
