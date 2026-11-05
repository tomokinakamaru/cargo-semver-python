import unittest

from cargo_semver import compare, matches, parse, validate


class SemverTests(unittest.TestCase):
    def test_parse(self):
        self.assertEqual(
            parse("1.2.3-alpha.1+build.5"),
            {
                "major": 1,
                "minor": 2,
                "patch": 3,
                "prerelease": "alpha.1",
                "build": "build.5",
            },
        )
        self.assertEqual(
            parse("0.0.0"),
            {
                "major": 0,
                "minor": 0,
                "patch": 0,
                "prerelease": None,
                "build": None,
            },
        )
        self.assertEqual(parse("1.2.3+001")["build"], "001")
        self.assertEqual(parse("18446744073709551615.0.0")["major"], 2**64 - 1)

    def test_parse_invalid(self):
        for value in ("1.2", "01.2.3", "1.2.3-alpha..1", "18446744073709551616.0.0"):
            with self.subTest(value=value):
                with self.assertRaises(ValueError):
                    parse(value)

    def test_validate(self):
        for value in ("0.0.0", "1.2.3", "1.2.3-alpha.1", "1.2.3+build.5"):
            with self.subTest(value=value):
                self.assertTrue(validate(value))

        for value in ("1.2", "01.2.3", "1.0.0-alpha..1", "1.0.0-01"):
            with self.subTest(value=value):
                self.assertFalse(validate(value))

    def test_compare(self):
        cases = (
            ("1.2.3", "1.2.4", -1),
            ("1.2.3", "1.2.3", 0),
            ("1.2.4", "1.2.3", 1),
            ("1.0.0-alpha", "1.0.0", -1),
            ("1.0.0-alpha.2", "1.0.0-alpha.11", -1),
            ("1.0.0+abc", "1.0.0+def", 0),
        )
        for left, right, expected in cases:
            with self.subTest(left=left, right=right):
                self.assertEqual(compare(left, right), expected)

    def test_matches(self):
        cases = (
            ("1.2.3", "^1.0", True),
            ("1.2.3", "1.2", True),
            ("2.0.0", "^1.0", False),
            ("1.0.0", "=1.0.0", True),
            ("1.5.0", ">=1.0, <2.0", True),
            ("2.0.0", ">=1.0, <2.0", False),
            ("0.2.5", "^0.2", True),
            ("0.3.0", "^0.2", False),
            ("1.2.3", "1.*", True),
            ("1.2.3-alpha.1", "^1.0", False),
            ("1.2.3-alpha.1", "^1.2.3-alpha.1", True),
            ("1.2.4-alpha.1", "^1.2.3-alpha.1", False),
            ("1.2.3-alpha.1", "*", False),
        )
        for version, requirement, expected in cases:
            with self.subTest(version=version, requirement=requirement):
                self.assertEqual(matches(version, requirement), expected)

    def test_invalid_inputs(self):
        for invoke in (
            lambda: compare("invalid", "1.2.3"),
            lambda: compare("1.2.3", "invalid"),
            lambda: matches("invalid", "^1.0"),
            lambda: matches("1.2.3", "invalid"),
        ):
            with self.subTest(invoke=invoke):
                with self.assertRaises(ValueError):
                    invoke()


if __name__ == "__main__":
    unittest.main()
