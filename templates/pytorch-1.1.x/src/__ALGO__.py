import Algorithmia
import torch as th

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
            raise Exception('inner dimensions between A and B must be the same.\n A: {} B: {}'.format(self.A.shape[-1],
                                                                                                      self.B.shape[0]))


def convert(list_array):
    """
    Converts a json list into a torch Tensor object.
    """
    th_tensor = th.tensor(list_array).float()
    gpu_tensor = th_tensor.cuda()
    return gpu_tensor

def apply(input):
    """
    Calculates the dot product of two matricies using pytorch, with a cudnn backend.
    Returns the product as the output.
    """
    input = InputObject(input)
    C = th.mm(input.A, input.B)
    z = C.cpu().numpy().tolist()
    output = {'product': z}
    return output
