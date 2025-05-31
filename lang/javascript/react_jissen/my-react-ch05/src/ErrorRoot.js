import { ErrorBoundary } from "react-error-boundary";
import ErrorThrow from "./ErrorThrow";

export default function ErrorRoot() {
    return (
        <>
            <h4>Error Boundaryの基本</h4>
            <ErrorBoundary fallback={<div>エラーが発生しました。</div>}>
                <ErrorThrow />
            </ErrorBoundary>
        </>
    );
}