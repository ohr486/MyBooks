import { Route, createBrowserRouter, createRoutesFromElements } from "react-router-dom";
import RouterParam from './RouterParam';
import TopPage2 from './TopPage2';
import BookPage from './BookPage';
import SearchPage from './SearchPage';
import NotFoundPage from './NotFoundPage';
import BookQueryPage from './BookQueryPage';
import BookStatePage from './BookStatePage';

const routesParam = createBrowserRouter(
    createRoutesFromElements(
        <Route element={<RouterParam />}>
            <Route path="/" element={<TopPage2 />} />
            <Route path="/book/:isbn?" element={<BookPage />} />
            <Route path="/bookQuery" element={<BookQueryPage />} />
            <Route path="/bookState" element={<BookStatePage />} />
            <Route path="/search/*" element={<SearchPage />} />
            <Route path="*" element={<NotFoundPage />} />
        </Route>
    )
);

export default routesParam;
