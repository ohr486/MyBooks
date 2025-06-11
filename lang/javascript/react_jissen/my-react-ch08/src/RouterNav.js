import { NavLink, Outlet } from 'react-router-dom';
import './RouterNav.css';

export default function RouterNav() {
    return (
        <>
            <ul>
                <li><NavLink to="/">トップ</NavLink></li>
                <li><NavLink to="/article">公開記事</NavLink></li>
                <li><NavLink to="/about">このサイトについて</NavLink></li>
            </ul>
            <hr />
            <Outlet />
        </>
    );
}