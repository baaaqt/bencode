from unittest import TestCase

from bencode import Bencoder


class TestBencoder(TestCase):
    def setUp(self) -> None:
        self.bencoder = Bencoder()

    def test_parse_empty_dict(self) -> None:
        data = b"de"
        result = self.bencoder.parse(data)
        self.assertEqual(result, {})

    def test_parse_empty_list(self) -> None:
        data = b"le"
        result = self.bencoder.parse(data)
        self.assertEqual(result, [])

    def test_parse_integer(self) -> None:
        data = b"i42e"
        result = self.bencoder.parse(data)
        self.assertEqual(result, 42)

    def test_parse_string(self) -> None:
        data = b"4:spam"
        result = self.bencoder.parse(data)
        self.assertEqual(result, b"spam")

    def test_parse_list_with_integers(self) -> None:
        data = b"li1ei2ei3ee"
        result = self.bencoder.parse(data)
        self.assertEqual(result, [1, 2, 3])

    def test_parse_list_with_strings(self) -> None:
        data = b"l4:spam4:eggse"
        result = self.bencoder.parse(data)
        self.assertEqual(result, [b"spam", b"eggs"])

    def test_parse_dict_with_string_keys(self) -> None:
        data = b"d3:cow3:moo4:spam4:eggse"
        result = self.bencoder.parse(data)
        self.assertEqual(result, {b"cow": b"moo", b"spam": b"eggs"})

    def test_parse_nested_structures(self) -> None:
        data = b"d4:spaml1:a1:bee"
        result = self.bencoder.parse(data)
        self.assertEqual(result, {b"spam": [b"a", b"b"]})

    def test_parse_complex_structure(self) -> None:
        data = (
            b"d9:publisher3:bob17:publisher-webpage15:www.example.com18:publisher.loc"
            b"ation4:homee"
        )
        result = self.bencoder.parse(data)
        self.assertEqual(
            result,
            {
                b"publisher": b"bob",
                b"publisher-webpage": b"www.example.com",
                b"publisher.location": b"home",
            },
        )

    def test_parse_negative_integer(self) -> None:
        data = b"i-42e"
        result = self.bencoder.parse(data)
        self.assertEqual(result, -42)

    def test_parse_zero(self) -> None:
        data = b"i0e"
        result = self.bencoder.parse(data)
        self.assertEqual(result, 0)
