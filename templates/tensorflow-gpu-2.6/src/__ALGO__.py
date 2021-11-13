import Algorithmia
import tensorflow.keras.backend as K
from tensorflow import convert_to_tensor
import tensorflow as tf
import numpy as np

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

# Print TensorFlow info
print(tf.__version__)
print(tf.test.is_built_with_cuda())
print(tf.sysconfig.get_build_info()["cuda_version"])
print(tf.test.is_built_with_gpu_support())
print(tf.config.list_physical_devices("GPU"))
print(tf.test.gpu_device_name())


# Configure Tensorflow to only use up to 30% of the GPU.
gpus = tf.config.experimental.list_physical_devices('GPU')
tf.config.experimental.set_memory_growth(gpus[0], True)
tf.config.experimental.set_virtual_device_configuration(gpus[0], [tf.config.experimental.VirtualDeviceConfiguration(memory_limit=3432)])


class InputObject:
    def __init__(self, input_dict):
        """
        Creates an instance of the InputObject, which checks the format of data and throws exceptions if anything is
        missing.
        "matrix_a" and "matrix_b" must be the same shape.

        :param A - Matrix A, converted from a json list into a keras Tensor.
        :param B - Matrix B, converted from a json list into a keras Tensor.
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
    Converts a json list into a keras Tensor object.
    """
    numpy_object = np.asarray(list_array, dtype=np.float)
    tensor_object = convert_to_tensor(numpy_object)
    return tensor_object


def apply(input):
    """
    Calculates the dot product of two matricies using keras, with a tensorflow-gpu backend.
    Returns the product as the output.
    """

    input = InputObject(input)

    z = K.dot(input.A, input.B)
    # Here you need to use K.eval() instead of z.eval() because this uses the backend session
    K.eval(z)
    z = K.get_value(z)
    output = {'product': z.tolist()}
    return output

