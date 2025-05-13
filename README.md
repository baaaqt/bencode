# bencoders
Bencode decoder for Python, implemented on Rust.

# Usage
```python
from bencoders import Bencoder

data = "ld4:spami42eee"

decoded = Bencoder().parse(data.encode())
assert [{b'spam':42}] == decoded
```
