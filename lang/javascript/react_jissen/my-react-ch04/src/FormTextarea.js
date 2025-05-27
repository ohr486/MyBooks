import { useState } from 'react';

export default function FormTextarea() {
    const [form, setForm] = useState({
        comment: 'さまざまなフォーム要素を...'
    });
    const handleForm = e => {
        setForm({
            ...form,
            [e.target.name]: e.target.value
        });
    };
    const show = () => {
        console.log(`コメント: ${form.comment}`);
    };

    return (
        <form>
            <label htmlFor="comment">コメント:</label><br />
            <textarea id="comment" name="comment"
                      cols="30" rows="7"
                      value={form.comment}
                      onChange={handleForm}></textarea><br />
            <button type="button" onClick={show}>
                送信
            </button>
        </form>
    );
}