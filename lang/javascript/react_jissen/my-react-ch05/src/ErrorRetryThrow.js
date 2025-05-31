export default function ErrorRetryThrow() {
    if (Math.random() < 0.6) {
        throw new Error('Error is occurred in MyApp.');
    }

    return (
        <p>正しく実行されました。</p>
    );
}