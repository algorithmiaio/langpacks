import Algorithmia
import torch
from . import helpers
from PIL import Image
import numpy as np

TARGET_IMAGE_SIZE = 224
CLASSES = ['cat', 'dog']
MODEL_PATH = "data://demo/pytorch_template/model.t7"

client = Algorithmia.client()


def get_model(client, model_path):
   local_file = client.file(model_path).getFile().name
   model = torch.jit.load(local_file)
   return model


def predict(model, image_path):
   image = Image.open(image_path)
   tensor = torch.Tensor(np.asarray(image))
   tensor = tensor.view(1, 3, TARGET_IMAGE_SIZE, TARGET_IMAGE_SIZE)
   result = model.forward(tensor)
   arged_max = np.argmax(result.detach().numpy()[0])
   pred_class = CLASSES[int(arged_max)]
   return pred_class



def get_image(client, image_url, image_dimensions):
   local_image_path = helpers.download_helpers.download_image(client, image_url, size=image_dimensions)
   return local_image_path


def apply(input):
   if isinstance(input, str):
       image_url = input
   elif isinstance(input, dict):
       if 'image_url' in input:
           image_url = helpers.download_helpers.type_check(input, 'image_url', str)
       else:
           raise Exception("'image_url' must be defined.")
   else:
       raise Exception("Input should be either a string or json object.")
   local_image = get_image(client, image_url, TARGET_IMAGE_SIZE)
   prediction = predict(model, local_image)
   output = {'prediction': prediction}
   return output


model = get_model(client, MODEL_PATH)

if __name__ == "__main__":
   input = {'image_url': "https://s3.amazonaws.com/algorithmia-uploads/money_cat.jpg"}
   result = apply(input)
   print(result)
