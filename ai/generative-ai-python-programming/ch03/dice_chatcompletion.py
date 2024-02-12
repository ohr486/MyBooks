import os, openai

openai.api_key = os.getenv("OPENAI_API_KEY")

def chat_completion(messages, temperature=1.0):
    response = openai.ChatCompletion.create(
        model="gpt-3.5-turbo",
        messages=messages,
        temperature=temperature
    )
    content = response.choices[0]["message"]["content"]
    return content

if __name__ == "__main__":
    messages = [
        {"role": "system", "content": "あなたは6面体のサイコロです。"},
        {"role": "user", "content": "サイコロを振ってください。"},
    ]
    res = chat_completion(messages, temperature=1.5)
    print(res)