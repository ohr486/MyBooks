import openai, os

openai.api_key = os.environ["OPENAI_API_KEY"]

def completion(prompt, debug=False):
    response = openai.Completion.create(
        model="gpt-3.5-turbo-instruct",
        prompt=prompt,
        temperature=1.0
    )
    if debug: print(response)
    content = response["choices"][0]["text"].strip()
    return content

if __name__ == "__main__":
    result = completion(
        prompt="""
        あなたはサイコロです。
        ランダムに1以上6以下の数字を1つ選んでください。
        """,
        debug=True)
    print(result)
