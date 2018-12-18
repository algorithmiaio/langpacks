import Algorithmia
from keras.models import load_model
from keras.preprocessing.image import img_to_array, load_img
from .helpers import download_helpers
import numpy as np

TARGET_IMAGE_DIMENSIONS = 224
CLASSES = ['cat', 'dog']
MODEL_PATH = "data://demo/keras_template/model.h5"

CLIENT = Algorithmia.client()


def get_model(client, model_path):
    local_file = client.file(model_path).getFile().name
    model = load_model(local_file)
    return model


MODEL = get_model(CLIENT, MODEL_PATH)


def predict(model, image_path, classes):
    image = load_img(image_path)
    tensor = img_to_array(image)
    tensor = tensor.reshape((1,) + tensor.shape)
    result = model.predict(tensor, verbose=1)
    arged_max = np.argmax(result[0])
    pred_class = classes[int(arged_max)]
    return pred_class


def get_image(client, image_url, image_dimensions):
    local_image_path = download_helpers.download_image(client, image_url, size=image_dimensions)
    return local_image_path


def apply(input, client=CLIENT, model=MODEL, image_dimensions=TARGET_IMAGE_DIMENSIONS, classes=CLASSES):
    if isinstance(input, str):
        image_url = input
    elif isinstance(input, dict):
        if 'image_url' in input:
            image_url = download_helpers.type_check(input, 'image_url', str)
        else:
            raise Exception("'image_url' must be defined.")
    else:
        raise Exception("Input should be either a string or json object.")
    local_image = get_image(client, image_url, image_dimensions)
    prediction = predict(model, local_image, classes)
    output = {'prediction': prediction}
    return output


if __name__ == "__main__":
    input = {'image_url': "https://i.imgur.com/j2kloCx.jpg"}
    result = apply(input, CLIENT)
    print(result)