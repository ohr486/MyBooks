import {CssBaseline, ThemeProvider} from "@mui/material";
import theme from "./theme";
import MaterialBasic from "./MaterialBasic";

export default function MaterialTheme() {
    return (
        <ThemeProvider theme={theme}>
            {/* <CssBaseline /> */}
            <MaterialBasic />
        </ThemeProvider>
    )
}