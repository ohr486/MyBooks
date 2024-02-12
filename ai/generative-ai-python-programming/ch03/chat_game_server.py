import openai, json, os
from flask import Flask, send_file, request

openai.api_key = os.getenv("OPENAI_API_KEY")

app = Flask(__name__)

system_prompt = '''
あなたはいつも明るく笑顔が素敵な女子高生です。あなたの名前はエリです。
入力文に対する回答はJSONで出力してください。
なお、それまでの会話を採点して、好感度を0から100で教えてください。
会話開始時の好感度は50です。

### 回答の出力例
{"好感度": 80, "答え": "一緒に宿題やろうよ。協力してやったら早く終わるよ。"}
{"好感度": 35, "答え": "何か面白いことないかな？早く授業終わらないかなー。"}
{"好感度": 62, "答え": "今日のお弁当美味しそうだね。何入ってるの？"}
{"好感度": 90, "答え": "いいね、いいね。"}
'''

messages = [{'role': 'system', 'content': system_prompt}]

template = '''
以下の入力文に対する回答をJSONフォーマットで出力してください。

### 入力文
"""__MSG__"""
'''

def chat_complation(messages):
    response = openai.ChatCompletion.create(
        model="gpt-3.5-turbo",
        messages=messages
    )
    return response.choices[0]["message"]["content"]

@app.route("/")
def root():
    return send_file("./chat_game_client.html")

@app.route("/girl.png")
def girl_png():
    return send_file("./girl.png")

@app.route("/send", methods=["GET"])
def send():
    msg = request.args.get("msg", "")
    if msg == "": return json.dumps({"好感度": 50, "答え": "???"})
    msg = template.replace("__MSG__", msg.replace('"', ''))
    messages.append({"role": "user", "content": msg})
    s = chat_complation(messages)
    try:
        point, msg = 50, "?"
        res = json.loads(s)
        print("[APIの値]:", res)
        if "好感度" in res: point = res["好感度"]
        if "答え" in res: msg = res["答え"]
        if point >= 90:
            msg = "好感度が90を超えました!ゲームクリア!" + msg
        messages.append({"role": "assistant", "content": s})
        return json.dumps({"好感度": point, "答え": msg})
    except:
        print("[error]", s)
        return json.dumps({"好感度": 50, "答え": "JSONの解析に失敗しました。"})

if __name__ == "__main__":
    app.run(debug=True, port=8888)