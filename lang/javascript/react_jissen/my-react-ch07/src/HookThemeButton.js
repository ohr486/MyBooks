import { useContext } from 'react';
import { Button } from '@mui/material';
import ThemeContext from './ThemeContext';

export default function HookThemeButton() {
    const { mode, toggleMode } = useContext(ThemeContext);
    return (
        <Button variang="contained" onClick={toggleMode}>
            Mode {mode}
        </Button>
    );
}