import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import reportWebVitals from './reportWebVitals';
//import App from './App';
//import './MyHello';
import MyHello from "./MyHello";
import EventBasic from "./EventBasic";
import StateBasic from "./StateBasic";
import books from './books';
import ForList from './ForList';
import ForNest from './ForNest';
import ForFilter from './ForFilter';
import ForSort from './ForSort';
import SelectStyle from "./SelectStyle";
import StyledPanel from './StyledPanel';
import TitledPanel from "./TitledPanel";
import TitledPanel2 from "./TitledPanel2";
import ListTemplate from "./ListTemplate";
import ListTemplate2 from "./ListTemplate2";
import TypeProp, { Member } from "./TypeProp";
import StateParent from "./StateParent";
import EventMouse from "./EventMouse";
import EventCompare from "./EventCompare";
import EventCompare2 from "./EventCompare2";
import EventError from "./EventError";
import EventObj from "./EventObj";
import EventPoint from "./EventPoint";
import EventKey from "./EventKey";

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
    <React.StrictMode>
    <>
        <h1>xxx</h1>

        <h1>EventKey</h1>
        <EventKey />

        <h1>EventPoint</h1>
        <EventPoint />

        <h1>EventObj</h1>
        <EventObj />

        <h1>EventError(correct path)</h1>
        <EventError src="./image/_wings.jpg" alt="サンプル画像" />

        <h1>EventError(invalid path)</h1>
        <EventError src="./image/wings.jpg" alt="サンプル画像" />

        <h1>EventCompare(mouseOver/mouseOut</h1>
        <EventCompare2 />

        <h1>EventCompare(mouseEnter/mouseLeave</h1>
        <EventCompare />

        <h1>EventMouse</h1>
        <EventMouse
            beforeSrc="https://wings.msn.to/books/978-4-7981-8949-9/978-4-7981-8949-9.jpg"
            afterSrc="https://wings.msn.to/books/978-4-297-14598-9/978-4-297-14598-9.jpg" />

        <h1>StateParent</h1>
        <StateParent />

        <h1>TypeProp</h1>
        <TypeProp prop1="hoge" />
        <TypeProp prop1={new Member()} />
        <TypeProp prop2="hoge" />
        <TypeProp prop3={new Member()} />
        <TypeProp prop4={[333, '鈴木花子']} />
        <TypeProp prop5={{ '鈴木花子': 15, '佐藤雫': '三十' }} />
        <TypeProp prop6={{ age: 35, sex: '女性' }} />

        <h1>MyHello with PropTypes</h1>
        <MyHello />

        <h1>ListTemplate2</h1>
        <ListTemplate2 src={books} render={ elem => (
            <>
                <dt>
                    <a href={`https://wings.msn.to/books/${elem.isbn}/${elem.isbn}.jpg`}>
                        {elem.title} ({elem.price}円)
                    </a>
                </dt>
                <dd>{elem.summary}</dd>
            </>
        )}/>

        <h1>ListTemplate</h1>
        <ListTemplate src={books}>
            {elem => (
                <>
                    <dt>
                        <a href={`https://wings.msn.to/books/${elem.isbn}/${elem.isbn}.jpg`}>
                            {elem.title} ({elem.price}円)
                        </a>
                    </dt>
                    <dd>{elem.summary}</dd>
                </>
            )}
        </ListTemplate>

        <h1>TitledPanel2</h1>
        <TitledPanel2>
            <p key="title">メンバー募集中!</p>
            <p key="body">ようこそ、WINGSプロジェクトへ</p>
        </TitledPanel2>

        <h1>TitledPanel</h1>
        <TitledPanel
            title={<p>メンバー募集中!</p>}
            body={<p>ようこそ、WINGSプロジェクトへ!</p>} />

        <h1>StyledPanel</h1>
        <StyledPanel>
            <p>メンバー募集中!</p>
            <p>ようこそ、WINGSプロジェクトへ!</p>
            <MyHello myName="鈴木" />
        </StyledPanel>

        <h1>SelectStyle</h1>
        <SelectStyle mode="light" />
        <SelectStyle mode="dark" />
        <SelectStyle mode="foo" />

        <h1>ForSort</h1>
        <ForSort src={books} />

        <h1>ForFilter</h1>
        <ForFilter src={books} />

        <h1>ForNest</h1>
        <ForNest src={books} />

        <h1>ForList</h1>
        <ForList src={books} />

        <h1>StateBasic</h1>
        <StateBasic init={0} />

        <h1>EventBasic</h1>
        <h2>type=time</h2><EventBasic type="time" />
        <h2>type=date</h2><EventBasic type="date" />
        <h2>type=''</h2><EventBasic />

        <h1>MyHello</h1>
        <MyHello myName="鈴木" />
        <MyHello myName={['山田', '鈴木', '佐藤']} />
        <MyHello />
    </>
    </React.StrictMode>
);

//<MyHello myName={{ name: '鈴木', age: 48 }} />

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
