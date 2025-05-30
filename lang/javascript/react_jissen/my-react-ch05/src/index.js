import ReactDOM from 'react-dom/client';
import './index.css';
import LazyBasic from './LazyBasic';
import LazyMulti from './LazyMulti';
import SuspenseSimple from './SuspenseSimple';
import SuspenseResult from './SuspenseResult';
import ProfilerBasic from './ProfilerBasic';
import StyledBasic from './StyledBasic';
import StyledCss from './StyledCss';
import StyledDynamic from './StyledDynamic';
import StyledComp from './StyledComp';
import { MyButton, MyStyledButton } from './StyledComp2';
import StyledCommon from './StyledCommon';
import GlobalStyle from './StyledGlobal';
import StyledProps from './StyledProps';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <>
    <h1>xxx</h1>
    <h1>xxx</h1>
    <h1>xxx</h1>
    <h1>xxx</h1>

    <h1>StyledProps</h1>
    <StyledProps />

    <h1>StyledGlobal</h1>
    <GlobalStyle />

    <h1>StyledCommon</h1>
    <StyledCommon />

    <h1>StyledComp2</h1>
    <>
      <MyButton>ボタン</MyButton>
      <MyStyledButton>ボタン</MyStyledButton>
    </>

    <h1>StyledComp</h1>
    <StyledComp />

    <h1>StyledDynamic</h1>
    <StyledDynamic theme={{
      radius: true,
      color: 'purple'
    }}/>

    <h1>StyledCss</h1>
    <>
      <h2>StyledCssの外部化</h2>
      <StyledCss />
    </>

    <h1>StyledBasic</h1>
    <>
      <h3>StyledBasicの基本</h3>
      <StyledBasic />
    </>

    <h1>ProfilerBasic</h1>
    <ProfilerBasic />

    <h1>SuspenseResult</h1>
    <SuspenseResult />

    <h1>SuspenseSimple</h1>
    <SuspenseSimple />

    <h1>LazyMulti</h1>
    <LazyMulti />

    <h1>LazyBasic</h1>
    <LazyBasic />
  </>
);
