import { forwardRef, useImperativeHandle, useRef } from 'react';

const MyTextBox = forwardRef(({ label }, ref) => {
    const input = useRef(null);

    useImperativeHandle(ref, () => {
        return {
            focus() {
                input.current.focus();
            },
        };
    }, []);

    return (
        <label>
            {label}:
            <input type="text" size="15" ref={input} />
        </label>
    );
});

export default MyTextBox;