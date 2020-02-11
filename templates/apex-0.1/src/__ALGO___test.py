from .__ALGO__ import apply


def test_algorithm():
    input = "Jane"
    result = apply(input)
    assert result == "Hello Jane!"
