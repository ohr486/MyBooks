import { Route, createBrowserRouter, createRoutesFromElements } from "react-router-dom";
import TopPage3 from './TopPage3';
import BookPage2 from './BookPage2';
import RouterParam2 from './RouterParam2';

const routesHandle = createBrowserRouter(
    createRoutesFromElements(
        <Route element={<RouterParam2 />}>
            <Route path="/" element={<TopPage3 />} handle={{
                title: 'トップ',
                keywords: 'React, Router, JavaScript',
                description: 'React Routerの解説サンプルです。'
            }} />
            <Route path="/book/:isbn?" element={<BookPage2 />} handle={{
                title: '書籍詳細 - %s',
                keywords: 'React %s',
                description: '%s'
            }} />
        </Route>
    )
);

export default routesHandle;