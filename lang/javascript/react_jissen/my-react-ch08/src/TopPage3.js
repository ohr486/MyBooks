import { useEffect } from 'react';
import { useOutletContext } from 'react-router-dom';
import MyHeader from './MyHeader';

export default function TopPage() {
    const [, setCount] = useOutletContext();
    useEffect(() => setCount(c => c + 1), [setCount]);

    return (
        <>
            <MyHeader />
            <p>トップページです。</p>
        </>
    );
}