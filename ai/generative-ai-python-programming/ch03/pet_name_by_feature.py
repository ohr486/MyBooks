import openai, os

openai.api_key = os.environ["OPENAI_API_KEY"]

def call_chatgpt(prompt, debug=False):
    response = openai.ChatCompletion.create(
        model="gpt-3.5-turbo",
        messages=[{"role": "user", "content": prompt}]
    )
    return response.choices[0]["message"]["content"]

feature = input("ペットの特徴を入力してください: ")
if feature == "": quit()

prompt = f"""
ペットの名前を3つ考えてください。
特徴: '''{feature}'''
"""

pet_names = call_chatgpt(prompt, debug=False)
print(pet_names)