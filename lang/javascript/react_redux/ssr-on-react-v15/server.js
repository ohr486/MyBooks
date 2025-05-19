import path from 'path';
import React from 'react';
import ReactDOMServer from 'react-dom/server';
import express from 'express';
import App from './App';

const app = express();

app.get('/client.bundle.js', (req, res) => {
    res.sendFile(path.join(__dirname + '/dist/client.bundle.js'));
})

//app.use('/static', express.static(__dirname + '/static'));

function HTML({ now, children }) {
    return (
        <html lang="ja">
            <head>
                <meta charSet="utf-8" />
                <title>シンプルなサーバーサイドレンダリング</title>
            </head>
            <body>
                {/* react15以前}
                <div id="root" dangerouslySetInnerHTML={{ __html: contents }}></div>
                */}

                {/* React16以降 */}
                <div id="root">{children}</div>

                <script
                    type="text/javascript"
                    id="server-now"
                    data-server-now={now.getTime() + ''}
                ></script>

                <script src="dist/client.bundle.js"></script>
            </body>
        </html>
    );
}

app.get('/', (req, res) => {
    const now = new Date();

    // React15以前
    //const contentsHTML = ReactDOMServer.renderToString(
    //    <App renderedAt={now} />
    //);
    //const fullHTML = ReactDOMServer.renderToStaticMarkup(
    //    <HTML contents={contentsHTML} now={now}/>
    //);
    //res.send(fullHTML);

    // React16以降
    const stream = ReactDOMServer.renderToNodeStream(
        <HTML now={now}>
            <App renderedAt={now} />
        </HTML>
    );
    stream.pipe(res);
});

app.listen(3000, () => {
    console.log('ポート3000番で起動...');
});
