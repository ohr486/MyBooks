import { useParams } from "react-router-dom";

export default function BookPage() {
    const { isbn = '978-4-8156-0182-9' } = useParams();
    return <p>ISBNコード [{isbn}] のページです。</p>;
}