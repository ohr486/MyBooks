import { Route, createBrowserRouter, createRoutesFromElements } from "react-router-dom";
import TopPage from './TopPage';
import ArticlePage from './ArticlePage';
import AboutPage from './AboutPage';

const routesBasic = createBrowserRouter(
    // [
    //     { path: '/', element: <TopPage /> },
    //     { path: '/articles', element: <ArticlePage /> },
    //     { path: '/about', element: <AboutPage /> },
    // ]
    createRoutesFromElements(
        <>
            <Route path="/" element={<TopPage />} />
            <Route path="/article" element={<ArticlePage />} />
            <Route path="/about" element={<AboutPage />} />
        </>
    )
);

export default routesBasic;