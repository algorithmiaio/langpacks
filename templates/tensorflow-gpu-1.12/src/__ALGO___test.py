from .__ALGO__ import apply

"""
This is the testing file, which should be bundled along with the Algorithm.py file when creating a template.
These tests are run during the creation of a template, and if they fail - the templating process fails as well.
"""

def primariy_api_example():
    input = {'image_url': 'https://s3.amazonaws.com/algorithmia-uploads/money_cat.jpg'}
    result = apply(input)
    return result

def test_api_change():
    response = primariy_api_example()
    assert 'prediction' in response
