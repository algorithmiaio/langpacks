import Algorithmia
import numpy as np
import os
from .auxillary import load, allocate_buffers, do_inference
from PIL import Image
# API calls will begin at the apply() method, with the request body passed as 'input'
# For more details, see algorithmia.com/developers/algorithm-development/languages

SIMD_ALGO = "util/SmartImageDownloader/0.2.14"
MODEL_PATH = "data://zeryx/collection/mobilenetv2-1.0.onnx"

client = Algorithmia.client("simP1oZ9sfF7c1cBrYUQ08iBtdP1")
trt_engine = load(client, MODEL_PATH)


def preprocess(image_path):
    image = Image.open(image_path)
    image_data = np.array(image).transpose(2, 0, 1)
    # convert the input data into the float32 input
    img_data = image_data.astype('float32')
    img_data = img_data.reshape(1, 3, 224, 224)

    #normalize
    mean_vec = np.array([0.485, 0.456, 0.406])
    stddev_vec = np.array([0.229, 0.224, 0.225])
    norm_img_data = np.zeros(img_data.shape).astype('float32')
    for i in range(img_data.shape[0]):
        norm_img_data[i,:,:] = (img_data[i,:,:]/255 - mean_vec[i]) / stddev_vec[i]
    return norm_img_data

def softmax(x):
    x = x.reshape(-1)
    e_x = np.exp(x - np.max(x))
    return e_x / e_x.sum(axis=0)

def postprocess(result):
    return softmax(np.array(result)).tolist()


def infer(image_data):
    inputs, outputs, bindings, stream = allocate_buffers(trt_engine)
    inputs[0].host = image_data
    with trt_engine.create_execution_context() as context:
        trt_outputs = do_inference(context, bindings=bindings, inputs=inputs, outputs=outputs, stream=stream)
    print(trt_outputs)
    return trt_outputs



def get_image(url, shape):
    output_url = client.algo(SIMD_ALGO).pipe({'image': str(url), "resize": shape}).result['savePath'][0]
    temp_file = client.file(output_url).getFile().name
    os.rename(temp_file, temp_file + '.' + output_url.split('.')[-1])
    local_file = temp_file + '.' + output_url.split('.')[-1]
    return local_file

def apply(input):
    image = get_image(input, {"height": 224, "width": 224})
    image_data = preprocess(image)
    trt_output = infer(image_data)
    res = postprocess(trt_output)
    return res
