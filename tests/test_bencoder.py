from unittest import TestCase

from pet import Bencoder


class TestBencoder(TestCase):
    def setUp(self) -> None:
        self.bencoder = Bencoder()

    def test_parse_empty_dict(self):
        data = b"de"
        result = self.bencoder.parse(data)
        self.assertEqual(result, {})

    def test_parse_empty_list(self):
        data = b"le"
        result = self.bencoder.parse(data)
        self.assertEqual(result, [])

    def test_parse_integer(self):
        data = b"i42e"
        result = self.bencoder.parse(data)
        self.assertEqual(result, 42)

    def test_parse_string(self):
        data = b"4:spam"
        result = self.bencoder.parse(data)
        self.assertEqual(result, b"spam")

    def test_parse_list_with_integers(self):
        data = b"li1ei2ei3ee"
        result = self.bencoder.parse(data)
        self.assertEqual(result, [1, 2, 3])

    def test_parse_list_with_strings(self):
        data = b"l4:spam4:eggse"
        result = self.bencoder.parse(data)
        self.assertEqual(result, [b"spam", b"eggs"])

    def test_parse_dict_with_string_keys(self):
        data = b"d3:cow3:moo4:spam4:eggse"
        result = self.bencoder.parse(data)
        self.assertEqual(result, {b"cow": b"moo", b"spam": b"eggs"})

    def test_parse_nested_structures(self):
        data = b"d4:spaml1:a1:bee"
        result = self.bencoder.parse(data)
        self.assertEqual(result, {b"spam": [b"a", b"b"]})

    def test_parse_complex_structure(self):
        data = b"d9:publisher3:bob17:publisher-webpage15:www.example.com18:publisher.location4:homee"
        result = self.bencoder.parse(data)
        self.assertEqual(
            result,
            {
                b"publisher": b"bob",
                b"publisher-webpage": b"www.example.com",
                b"publisher.location": b"home",
            },
        )

    def test_parse_negative_integer(self):
        data = b"i-42e"
        result = self.bencoder.parse(data)
        self.assertEqual(result, -42)

    def test_parse_zero(self):
        data = b"i0e"
        result = self.bencoder.parse(data)
        self.assertEqual(result, 0)
