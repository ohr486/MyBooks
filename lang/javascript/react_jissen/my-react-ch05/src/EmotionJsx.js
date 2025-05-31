/** @jsxImportSource @emotion/react */
import { css } from '@emotion/react';

export default function EmotionJsx() {
    //const styles = css`
    //    width: 300px;
    //    padding: 10px;
    //    border: 1px solid #000;
    //    border-radius: 5px;
    //    background-color: royalblue;
    //    color: white;
    //`;

    const styles = css({
        width: 300,
        padding: 10,
        border: '1px solid #000',
        borderRadius: 5,
        backgroundColor: 'royalblue',
        color: 'white',
    });

    return (
        <div css={styles}>
            <b>Styled JSX</b>は、JSX式にスタイル定義を...
        </div>
    );
}