from Algorithmia import ADK
from datarobot.mlops.mlops import MLOps
import torch
from torchvision import transforms
from time import time
import pandas as pd
from PIL import Image
from src.labels import labels_map
import os

# The model itself is stored in an Algorithmia Data Collection
def load(state):
    # We load the model in this way to ensure that the model is loaded only once
    model_path = state.client.file(os.environ['MODEL_PATH']).getFile(as_path=True)
    state['model'] = torch.jit.load(model_path)
    state['mlops'] = MLOps().init()
    state['labels'] = labels_map
    return state


# As we're using the ADK system, state is a dictionary that contains the model and the labels
# Input is expected to be an image file on the Algorithmia Data API; more information on the Data API can be found here.
# https://algorithmia.com/developers/data/hosted
def apply(input, state):
    start_t = time()
    local_img = state.client.file(input).getFile(as_path=True)
    transform = transforms.Compose([transforms.Resize(28),
                                    transforms.ToTensor()])
    img = Image.open(local_img).convert('L')
    tensor = transform(img)
    output = state['model'].forward(tensor)
    _, predicted = torch.max(output.data, 1)
    output = torch.softmax(output, 0).reshape(1, -1)
    prediction = state['labels'][int(predicted.item())]
    end_t = time()
    state['mlops'].report_deployment_stats(1, end_t - start_t)
    state['mlops'].report_predictions_data(predictions=output.tolist())
    return prediction


algorithm = ADK(apply, load)
algorithm.init(mlops=True)
