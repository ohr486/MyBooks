import { ErrorBoundary } from "react-error-boundary";
import ErrorRetryThrow from "./ErrorRetryThrow";
import ErrorFallback from "./ErrorFallback";

export default function ErrorRetryRoot() {
    const handleReset = () => console.log('Retry!!');

    return (
        <>
            <h2>Error Boundaryの基本</h2>
            <ErrorBoundary
                onReset={handleReset}
                FallbackComponent={ErrorFallback}>
                <ErrorRetryThrow />
            </ErrorBoundary>
        </>
    );
}