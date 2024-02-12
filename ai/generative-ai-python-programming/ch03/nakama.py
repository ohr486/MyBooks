import openai, os, json

openai.api_key = os.getenv("OPENAI_API_KEY")

template = '''
私は桃太郎であなたを仲間にしようと説得します。
鬼ヶ島へ鬼退治に行きたいのですが、仲間になってくれますか？

### 条件
- 仲間になるなら結果にtrueを、嫌ならfalseを返します。
- 説得内容に「きび団子」があれば{"結果": false, "理由":"食べ飽きている"}と返します。

### 応答の例
{"結果": false, "理由": "興味がないから"}
{"結果": true, "理由": "志に共感したため"}
{"結果": false, "理由": "きび団子になんかには釣られないよ"}

###説得内容
"""__MSG__"""
'''

def chat_completion(messages):
    response = openai.ChatCompletion.create(
        model="gpt-3.5-turbo",
        messages=messages
    )
    return response.choices[0]["message"]["content"]

print("犬を見つけました。犬を仲間にしたいので説得しましょう!")
while True:
    print("---")
    msg = input(">>> ")
    prompt = template.replace("__MSG__", msg.replace('"', ''))
    messages = [
        {"role": "system", "content": "あなたは強情な犬です。JSONで応答してください。"},
        {"role": "user", "content": prompt}
    ]
    res = {"結果": False, "理由": "不明"}
    s = chat_completion(messages)
    try:
        res = json.loads(s)
    except:
        print("[エラー] JSONの解析に失敗しました。", s)

    if ("結果" in res) and ("理由" in res) and (res["結果"]):
        print("犬は仲間になってくれました!")
        print("理由は..." + res["理由"] + "。")
        print("ゲームクリア!")
        break
    else:
        reason = res["理由"] if "理由" in res else "なし"
        print("残念。犬に断られました。理由は..." + reason + "。")
        print("引き続き説得しましょう。")
