import css from 'styled-jsx/css';

export default css` 
  .panel {
    width: 300px;
    padding: 10px;
    border: 1px solid royalblue;
    border-radius: 5px;
    background-color: royalblue;
    color: white;
  }`;

export const globalCss = css.global`
  h2 {
    background-color: green;
  }`;

export const resolveCss = css.resolve`
  .panel {
    margin: 50px;
  }`;