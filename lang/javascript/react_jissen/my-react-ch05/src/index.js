/** @jsxImportSource @emotion/react */

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
import EmotionJsx from './EmotionJsx';
import EmotionComp from './EmotionComp';
import PortalBasic from './PortalBasic';
import ErrorRoot from './ErrorRoot';
import ErrorRetryRoot from './ErrorRetryRoot';
import ErrorRetryRoot2 from './ErrorRetryRoot2';
import ErrorEventRoot from './ErrorEventRoot';

import { css, Global } from '@emotion/react';
const global = css`
    body {
        background-color: white;
    }
`;

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <>
    <h1>ErrorEventRoot</h1>
    <ErrorEventRoot />

    <h1>ErrorRetryRoot2</h1>
    <ErrorRetryRoot2 />

    <h1>ErrorRetryRoot</h1>
    <ErrorRetryRoot />

    <h1>ErrorRoot</h1>
    <ErrorRoot />

    <h1>PortalBasic</h1>
    <div id="dialog"></div>
    <PortalBasic />

    <h1>Global</h1>
    <>
      <Global styles={global} />
      <EmotionJsx />
    </>

    <h1>EmotionComp</h1>
    <EmotionComp />

    <h1>EmotionJsx</h1>
    <EmotionJsx />

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
