import { useEffect, useLayoutEffect, useState } from 'react';

const sleep = delay => {
    //const start = Date.now();
    let count = 0;
    while (true) {
        // if (Date.now - start > delay) { break; }
        if (count > delay) {
            break;
        } else {
            count = count + 1;
        }
    }
};

export default function HookEffect({ init }) {
    const [count, setCount] = useState(0);

    //useEffect(() => {
    useLayoutEffect(() => {
        sleep(2000);
        setCount(init);
    }, []);

    const handleClick = () => {
        setCount(c => c + 1);
    };

    return (
        <>
            <button onClick={handleClick}>カウント</button>
            <p>{count}回、クリックされました</p>
        </>
    );
}