import numpy as np
import re

class SWEEncoder_ja:
    def __init__(self, bpe, emoji):
        self.bpe = [[b] if (b == ',' or ',' not in b) else b.split(',') for b in bpe]
        self.swe = {}
        self.emoji = emoji