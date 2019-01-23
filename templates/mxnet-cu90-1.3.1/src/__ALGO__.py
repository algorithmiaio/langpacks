import Algorithmia
from Algorithmia.errors import AlgorithmException
import torch
# from src.helpers import download_helpers
from PIL import Image
import numpy as np
import mxnet as mx


import json
from mxnet import gluon, nd
from mxnet.gluon.model_zoo import vision


ctx = mx.gpu()
mobileNet = vision.mobilenet0_5(pretrained=True, ctx=ctx)

