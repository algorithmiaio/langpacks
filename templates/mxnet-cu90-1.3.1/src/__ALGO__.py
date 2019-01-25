import Algorithmia
import mxnet as mx

CTX = mx.gpu()


class InputObject:
    def __init__(self, input_dict):
        """
        Creates an instance of the InputObject, which checks the format of data and throws exceptions if anything is
        missing.
        "matrix_a" and "matrix_b" must be the same shape.
        :param A - Matrix A, converted from a json list into an mxnet Tensor.
        :param B - Matrix B, converted from a json list into an mxnet Tensor.
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
    Converts a json list into a mxnet Tensor object.
    """
    mx_tensor = mx.nd.array(list_array)
    return mx_tensor

def apply(input):
    """
    Calculates the dot product of two matricies using mxnet, with cudnn backend.
    Returns the product as the output.
    """
    input = InputObject(input)
    C = mx.nd.dot(input.A, input.B)
    z = C.as_in_context(CTX).asnumpy().tolist()
    output = {'product': z}
    return output
