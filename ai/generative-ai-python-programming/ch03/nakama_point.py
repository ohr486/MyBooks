import openai, os, json

openai.api_key = os.getenv("OPENAI_API_KEY")

template = '''
次の題の文章について、論理的かどうか、ユニークかどうかを0から100で採点してください。

### 題
- 桃太郎が鬼退治に行く仲間を探す

### 応答の例
{"論理":80, "ユニーク": 30, "論評": "論理的だが、ありふれた内容で、心が動かない"}
{"論理":50, "ユニーク": 90, "論評": "論理的ではないが、ユニークで面白い"}

### 文章
"""__MSG__"""
'''

def chat_completion(messages):
    response = openai.ChatCompletion.create(
        model="gpt-3.5-turbo",
        messages=messages
    )
    return response.choices[0]["message"]["content"]

point = 0
print("犬を見つけました。犬を仲間にしたいので説得しましょう。")
while True:
    msg = input(">>> ")
    prompt = template.replace("__MSG__", msg.replace('"', ''))
    messages = [
        {"role": "system", "content": "JSONで応答してください。"},
        {"role": "user", "content": prompt}
    ]
    s = chat_completion(messages)
    try:
        logic, unique, comment = 0, 0, '?'
        res = json.loads(s)
        if "論理" in res: logic = res["論理"]
        if "ユニーク" in res: unique = res["ユニーク"]
        if "論評" in res: comment = res["論評"]
        point += logic + unique
    except:
        print("[エラー] JSONの解析に失敗しました。", s)
        continue

    print(f'論理: {logic}点, ユニーク: {unique}点 → {comment}')
    print(f"--- 合計得点: {point} ---")
    if point >= 300:
        print("犬が仲間になってくれました!")
        print("ゲームクリア!")
        break
    else:
        print("引き続き説得しましょう。")
