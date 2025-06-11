import { NavLink, Outlet } from 'react-router-dom';
import './RouterNav.css';

const isCurrent = ({ isActive }) => isActive ? {
    color: 'Blue',
    fontWeight: 'bold'
} : {};

export default function RouterNav() {
    return (
        <>
            <ul>
                <li><NavLink style={isCurrent} to="/">トップ</NavLink></li>
                <li><NavLink style={isCurrent} to="/article">公開記事</NavLink></li>
                <li><NavLink style={isCurrent} to="/about">このサイトについて</NavLink></li>
            </ul>
            <hr />
            <Outlet />
        </>
    );
}