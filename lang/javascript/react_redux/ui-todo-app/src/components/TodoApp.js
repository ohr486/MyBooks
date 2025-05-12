import React from "react";
import CssBaseline from '@mui/material/CssBaseline';
import AppBar from '@mui/material/AppBar';
import ToolBar from '@mui/material/Toolbar';
import Typography from "@mui/material/Typography";
import Button from '@mui/material/Button';
import Input from '@mui/material/Input';
import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import ListItemText from '@mui/material/ListItemText';
import { CSSTransition, TransitionGroup } from 'react-transition-group';
import './TodoApp.css';

export default function TodoApp({ task, tasks, inputTask, addTask }) {
    return (
        <div>
            <CssBaseline />
            <AppBar position="static">
                <ToolBar>
                    <Typography type="title" color="inherit">
                        Todo
                    </Typography>
                </ToolBar>
            </AppBar>
            <div style={{ padding: '16px' }}>
                <Input onChange={(e) => inputTask(e.target.value)} />
                <Button variant="contained" color="secondary" onClick={() => addTask(task)}>add</Button>
                <TransitionGroup>
                    <List>
                        {
                            tasks.map(function(item, i) {
                                return (
                                    <CSSTransition key={i} timeout={300} classNames="fade">
                                        <ListItem>
                                            <ListItemText primary={`・${item}`}/>
                                        </ListItem>
                                    </CSSTransition>
                                );
                            })
                        }
                    </List>
                </TransitionGroup>
            </div>
        </div>
    );
}