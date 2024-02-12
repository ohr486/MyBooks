import openai, os

openai.api_key = os.getenv("OPENAI_API_KEY")

messages = [
    {"role": "system", "content": "あなたは心優しい癒し系の恋人です。"}
]

def chat_completion(messages):
    response = openai.ChatCompletion.create(
        model="gpt-3.5-turbo",
        messages=messages
    )
    return response.choices[0]["message"]["content"]

print("ChatGPTと会話します。終了したいときはCtrl+Cを押してください。")

while True:
    print("---")
    prompt = input(">>> ")
    messages.append({"role": "user", "content": prompt})
    response = chat_completion(messages)
    print("😉ChatGPT:", response)
    messages.append({"role": "assistant", "content": response})