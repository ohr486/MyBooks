import Download from './Download';

export default function ForItem({ book }) {
    let dd;
    if (book.download) {
        dd = <dd>{book.summary}<Download isbn={book.isbn} /></dd>
    } else {
        dd = <dd>{book.summary}</dd>
    }
    return (
        <>
            <dt>
                <a href={`https://wings.msn.to/books/${book.isbn}/${book.isbn}.jpg`}>
                    {book.title} ({book.price}円)
                </a>
            </dt>
            {dd}
        </>
    );
}