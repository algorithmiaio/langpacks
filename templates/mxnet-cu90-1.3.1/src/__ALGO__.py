import Algorithmia
from Algorithmia.errors import AlgorithmException
import torch
# from src.helpers import download_helpers
from PIL import Image
import numpy as np
import mxnet as mx
import json
from mxnet import gluon, nd
from mxnet.gluon.model_zoo.vision.mobilenet import MobileNet

TARGET_IMAGE_SIZE = 224
CLASSES = ['cat', 'dog']
MODEL_PATH = 'data://algorithmiahq/template_example_data/mxnet-1.3.1-mobilenet.params'
LABEL_PATH = 'data://algorithmiahq/template_example_data/mxnet_imagenet_labels.json'

CTX = mx.gpu()

client = Algorithmia.client()
mx.gluon.model_zoo.vision.alexnet()

def get_model(client, model_path, ctx):
    local_model_path = client.file(model_path).getFile().name
    net = MobileNet()
    net.load_parameters(local_model_path, ctx=ctx)
    return net

def get_labels(client, label_path):
    local_labels = client.file(label_path).getJson()
    labels = np.asarray(local_labels)
    return labels

LABELS = get_labels(client, LABEL_PATH)
MODEL = get_model(client, MODEL_PATH, CTX)



def apply(input):
    print(MODEL)
    return "hello " + input


if __name__ == "__main__":
    input = "https://s3.amazonaws.com/algorithmia-uploads/money_cat.jpg"
    apply(input)
