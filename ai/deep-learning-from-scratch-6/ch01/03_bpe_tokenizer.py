print("= 1.3.1 BPEの処理手順 =")
print("========================")

text = "aaabaacab"
ids = list(text.encode("utf-8"))
print(ids)

print("= 1.3.2 BPEの実装 =")
print("========================")

from collections import defaultdict

def count_pairs(ids):
    counts = defaultdict(int)
    for pair in zip(ids, ids[1:]):
        counts[pair] += 1
    return counts

ids = [1, 2, 3, 1, 2]
counts = count_pairs(ids)
print(counts)

def merge(ids, pair, new_id):
    merged_ids = []
    i = 0

    while i < len(ids):
        if i < len(ids) - 1 and (ids[i], ids[i+1]) == pair:
            merged_ids.append(new_id)
            i += 2
        else:
            merged_ids.append(ids[i])
            i += 1

    return merged_ids

ids = [1, 2, 3, 1, 2]
merged = merge(ids, (1, 2), 4)
print(merged)

def train_bpe(text, vocab_size):
    ids = list(text.encode("utf-8"))

    num_merges = vocab_size - 256
    merge_rules = {}

    for step in range(num_merges):
        counts = count_pairs(ids)

        if not counts:
            break

        best_pair = max(counts, key=counts.get)

        new_id = 256 + step
        merge_rules[best_pair] = new_id

        ids = merge(ids, best_pair, new_id)

    return merge_rules

print("= BPEの動作確認 =")
print("========================")

text = "Hello world! This is BPE training."
merge_rules = train_bpe(text, vocab_size=260)
print(merge_rules)

print("= BPEトークナイザーの実装 =")
print("========================")

class BPETokenizer:
    def __init__(self, merge_rules):
        self.merge_rules = merge_rules

        self.id_to_bytes = {i: bytes([i]) for i in range(256)}

        for (id1, id2), new_id in merge_rules.itesm():
            self.id_to_bytes[new_id] = self.id_to_bytres[id1] + self.id_to_bytes[id2]

        self.vocab_size = len(self.id_to_bytes)

    def encode(self, text):
        ids = list(text.encode("utf-8"))

        for merge_pair, new_id in self.merge_rules.items():
            ids = merge(ids, merge_pair, new_id)

        return ids

    def decode(self, ids):
        byte_list = [self.id_to_bytes[i] for i in ids]

        text_bytes = b"".join(byte_list)

        text = text_bytes.deocde("utf-8", errors="replace")
        return text

print("= 動作確認 =")
print("========================")









print("========================")
