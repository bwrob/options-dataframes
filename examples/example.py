"""Test options_dataframes package."""

import inspect

import options_dataframes as odf
import polars as pl


def test_options_dataframes() -> None:
    """Test options_dataframes package."""
    odf_members = inspect.getmembers(odf)
    odf_functions = [
        member[0] for member in odf_members if inspect.isbuiltin(member[1])
    ]
    print(odf_functions)

    print(odf.sum_as_string(1, 2))
    print(odf.factorial(5))

    a = pl.Series("a", ["foo", "bar"])
    b = pl.Series("b", ["fooy", "ham"])

    dist = odf.hamming_distance(a, b)
    print(dist)
    expected = pl.Series("", [None, 2], dtype=pl.UInt32)
    print(expected)


if __name__ == "__main__":
    test_options_dataframes()
