import Algorithmia
from argparse import Namespace
import os
import sys
import pprint
import numpy as np
from PIL import Image
import torch
import ninja
import torchvision.transforms as transforms
from datasets.augmentations import AgeTransformer
from utils.common import tensor2im
from models.psp import pSp

"""
Example Input:
{
    "matrix_a": [[0, 1], [1, 0]],
    "matrix_b": [[25, 25], [11, 11]]
}

Expected Output:
{
    "product": [[11, 11], [25, 25]]
}
"""

class InputObject:
    def __init__(self, input_dict):
        """
        Creates an instance of the InputObject, which checks the format of data and throws exceptions if anything is
        missing.
        "matrix_a" and "matrix_b" must be the same shape.
        :param A - Matrix A, converted from a json list into a torch cuda Tensor.
        :param B - Matrix B, converted from a json list into a torch cuda Tensor.
        """
        if isinstance(input_dict, dict):
            if {'matrix_a', 'matrix_b'} <= input_dict.keys():
                self.A = convert(input_dict['matrix_a'])
                self.B = convert(input_dict['matrix_b'])
            else:
                raise Exception("'matrix_a' and 'matrix_b' must be defined.")
        else:
            raise Exception('input must be a json object.')
        if self.A.shape[-1] != self.B.shape[0]:
            raise Exception('inner dimensions between A and B must be the same.\n A: {} B: {}'
                .format(self.A.shape[-1], self.B.shape[0]))


def convert(list_array):
    """
    Converts a json list into a torch Tensor object.
    """
    th_tensor = torch.tensor(list_array).float()
    gpu_tensor = th_tensor.cuda()
    return gpu_tensor

client = Algorithmia.client("sim0/CA0mCa6Xz3FAkyoHb45G5I1")
EXPERIMENT_TYPE = 'ffhq_aging'
EXPERIMENT_DATA_ARGS = {
    "ffhq_aging": {
        "model_path": client.file("data://asli/aging/sam_ffhq_aging.pt").getFile().name,
        "image_path": "./notebooks/images/1287.jpg",
        "transform": transforms.Compose([
            transforms.Resize((256, 256)),
            transforms.ToTensor(),
            transforms.Normalize([0.5, 0.5, 0.5], [0.5, 0.5, 0.5])])
    }
}

EXPERIMENT_ARGS = EXPERIMENT_DATA_ARGS[EXPERIMENT_TYPE]
model_path = EXPERIMENT_ARGS['model_path']
ckpt = torch.load(model_path, map_location='cpu')
opts = ckpt['opts']
pprint.pprint(opts)
opts['checkpoint_path'] = model_path
opts = Namespace(**opts)
net = pSp(opts)
net.eval()
net.cuda()
print('Model successfully loaded!')

def run_alignment(image_path):
    import dlib
    from scripts.align_all_parallel import align_face
    file_path = client.file("data://asli/aging/shape_predictor_68_face_landmarks.dat").getFile().name
    predictor = dlib.shape_predictor(file_path)
    aligned_image = align_face(filepath=image_path, predictor=predictor)
    print("Aligned image has shape: {}".format(aligned_image.size))
    return aligned_image

def run_on_batch(inputs, net):
    result_batch = net(inputs.to("cuda").float(), randomize_noise=False, resize=False)
    return result_batch

def apply(input):
    image_path = EXPERIMENT_DATA_ARGS[EXPERIMENT_TYPE]["image_path"]
    original_image = Image.open(image_path).convert("RGB")
    original_image.resize((256, 256))
    aligned_image = run_alignment(image_path)
    aligned_image.resize((256, 256))
    img_transforms = EXPERIMENT_ARGS['transform']
    input_image = img_transforms(aligned_image)

    target_ages = [0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100]
    age_transformers = [AgeTransformer(target_age=age) for age in target_ages]

    # for each age transformed age, we'll concatenate the results to display them side-by-side
    results = np.array(aligned_image.resize((1024, 1024)))
    for age_transformer in age_transformers:
        print(f"Running on target age: {age_transformer.target_age}")
        with torch.no_grad():
            input_image_age = [age_transformer(input_image.cpu()).to('cuda')]
            input_image_age = torch.stack(input_image_age)
            result_tensor = run_on_batch(input_image_age, net)[0]
            result_image = tensor2im(result_tensor)
            results = np.concatenate([results, result_image], axis=1)
    results = Image.fromarray(results)
    #results.save("notebooks/images/age_transformed_image.jpg")
    """
    Calculates the dot product of two matricies using pytorch, with a cudnn backend.
    Returns the product as the output.
    input = InputObject(input)
    C = th.mm(input.A, input.B)
    z = C.cpu().numpy().tolist()
    """
    output = {'success': 1}
    return output
