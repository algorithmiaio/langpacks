from .__ALGO__ import apply


def test_algorithm():
    input = {'matrix_a': [[0, 1], [1, 0]], 'matrix_b': [[25, 25], [11, 11]]}
    result = apply(input)
    assert result['product'] == [[25., 25.], [11., 11.]]
