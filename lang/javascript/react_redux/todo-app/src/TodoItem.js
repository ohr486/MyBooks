import React, { Component } from 'react';

function TodoItem(props) {
    return (
        <li>
            {props.title}
        </li>
    )
}

export default TodoItem;