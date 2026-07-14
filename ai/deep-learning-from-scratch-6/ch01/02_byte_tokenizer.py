print("= 1.2.1 UTF-8エンコード =")
print("========================")

encoded = 'A'.encode("utf-8")
print(encoded)
print(list(encoded))

encoded = 'あ'.encode("utf-8")
print(encoded)
print(list(encoded))

ids = [65]
decoded = bytes(ids).decode("utf-8")
print(decoded)

print("= 1.2.2 バイト単位トークナイザの実装 =")
print("========================")

class ByteTokenizer:
    def encode(self, text):
        return list(text.encode("utf-8"))

    def decode(self, ids):
        return bytes(ids).decode("utf-8")

tokenizer = ByteTokenizer()
text = "hello世界😃"

ids = tokenizer.encode(text)
print(ids)

decoded = tokenizer.decode(ids)
print(decoded)

