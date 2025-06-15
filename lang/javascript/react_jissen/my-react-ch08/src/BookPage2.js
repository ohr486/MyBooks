import { useParams } from "react-router-dom";
import MyHeader from './MyHeader';
import books from './books';

export default function BookPage2() {
    const { isbn = '978-4-8156-0182-9' } = useParams();
    const { title, summary } = books.find(b => isbn === b.isbn);
    return (
        <>
            <MyHeader title={title} keywords={title} description={summary} />
            <p>ISBNコード [{isbn}] のページです。</p>
        </>
    );
}