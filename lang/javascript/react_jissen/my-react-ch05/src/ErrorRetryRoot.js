import { ErrorBoundary } from "react-error-boundary";
import ErrorRetryThrow from "./ErrorRetryThrow";

export default function ErrorRetryRoot() {
    const handleFallback = ({ error, resetErrorBoundary }) => {
        const handleClick = () => resetErrorBoundary();
        return (
            <div>
                <h4>以下のエラーが発生しました。</h4>
                <p>{error.message}</p>
                <button type="button" onClick={handleClick}>
                    Retry
                </button>
            </div>
        );
    };

    const handleReset = () => console.log('Retry!!');

    return (
        <>
            <h2>Error Boundaryの基本</h2>
            <ErrorBoundary
                onReset={handleReset}
                fallbackRender={handleFallback}>
                <ErrorRetryThrow />
            </ErrorBoundary>
        </>
    );
}