from .__ALGO__ import apply


def test_algorithm():
    input = "I like New York in Autumn, the trees are beautiful."
    result = apply(input)
    assert result == {"entities found": [{"label": "GPE", "text": "New York"}, {"label": "DATE", "text": "Autumn"}]}
