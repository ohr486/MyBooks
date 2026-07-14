print("= 1.1.1 文字をIDに変換する =")
print("========================")

text = "hello世界😃"
print(list(text))

print("========================")

print(ord('h'))
print(ord('😃'))

print("========================")

text = "hello世界😃"
ids = [ord(char) for char in list(text)]
print(ids)

print("= 1.1.2 文字単位トークナイザの実装 =")
print("========================")

class CharTokenizer:
    def encode(self, text):
        return [ord(char) for char in text]

    def decode(self, ids):
        return ''.join([chr(i) for i in ids])

chars = ['h', 'e', 'l', 'l', 'o']
print(''.join(chars))
print('-'.join(chars))

tokenizer = CharTokenizer()
text = "hello世界😃"

ids = tokenizer.encode(text)
print(ids)

decoded = tokenizer.decode(ids)
print(decoded)

