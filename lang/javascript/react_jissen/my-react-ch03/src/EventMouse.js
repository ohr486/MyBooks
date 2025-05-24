import { useState } from 'react';

export default function EventMouse({ beforeSrc, afterSrc, alt }) {
    const [current, setCurrent] = useState(beforeSrc);

    const handleEnter = () => setCurrent(afterSrc);
    const handleLeave = () => setCurrent(beforeSrc);

    return (
        <img src={current} alt={alt} height="300"
             onMouseEnter={handleEnter} onMouseLeave={handleLeave} />
    );
}