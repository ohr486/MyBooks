/** @jsxImportSource @emotion/react */
import { css } from '@emotion/react';
import { Button } from '@mui/material';

export default function MaterialBasic() {
    const font = css`
      text-transform: none;
    `;
    return (
        <>
            <Button variant="text" color="secondary" css={font}>Text</Button>
            <Button variant="contained" color="secondary">Contained</Button>
            <Button variant="outlined" color="secondary">Outlined</Button>
        </>
    );
}