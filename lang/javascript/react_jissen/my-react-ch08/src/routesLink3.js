import { Route, createBrowserRouter, createRoutesFromElements } from 'react-router-dom';
import RouterNav2 from './RouterNav2';
import TopPage from './TopPage';
import ArticlePage from './ArticlePage';
import AboutPage from './AboutPage';

const routesLink = createBrowserRouter(
    createRoutesFromElements(
        <Route element={<RouterNav2 />}>
            <Route path="/" element={<TopPage />} />
            <Route path="/article" element={<ArticlePage />} />
            <Route path="/about" element={<AboutPage />} />
        </Route>
    )
);

export default routesLink;
