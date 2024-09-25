"""Test options_dataframes package."""

import inspect

import options_dataframes as odf


def test_options_dataframes() -> None:
    """Test options_dataframes package."""
    odf_members = inspect.getmembers(odf)
    print(odf_members)
    odf_functions = [
        member[0] for member in odf_members if inspect.isbuiltin(member[1])
    ]
    print(odf_functions)

    print(odf.sum_as_string(1, 2))
    print(odf.factorial(5))


if __name__ == "__main__":
    test_options_dataframes()
