from .__ALGO__ import apply

def test_algorithm():
    input = "jane"
    result = apply(input)
    assert result == "hello jane"
