# bencoders

A Python library of the Bencode encoding format, widely used in BitTorrent protocol implemented in Rust.

# Usage
```python
from bencoders import Bencoder

data = "ld4:spami42eee"

decoded = Bencoder().parse(data.encode())
assert [{b'spam':42}] == decoded
```
