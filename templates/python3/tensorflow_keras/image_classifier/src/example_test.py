from .example import apply, get_model
import Algorithmia

"""
This is the testing file, which should be bundled along with the Algorithm.py file when creating a template.
These tests are run during the creation of a template, and if they fail - the templating process fails as well.
"""

TARGET_IMAGE_DIMENSIONS = 224
CLASSES = ['cat', 'dog']
MODEL_PATH = "data://demo/keras_template/model.h5"

def primariy_api_example():
    client = Algorithmia.client('YOUR_API_KEY_HERE')
    model_testing = get_model(client, MODEL_PATH)
    input = {'image_url': "https://i.imgur.com/j2kloCx.jpg"}
    result = apply(input, model=model_testing, client=client, image_dimensions=TARGET_IMAGE_DIMENSIONS, classes=CLASSES)
    return result

def test_api_change():
    response = primariy_api_example()
    assert "prediction" in response
