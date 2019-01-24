
import tensorflow.keras.backend as K
import numpy as np

def apply(input):
    """
    Multiplies two matricies together using keras, with a tensorflow-gpu backend.
    Returns the result as the output.
    """
    A = np.random.rand(10, 500)
    B = np.random.rand(500, 6000)

    x = K.variable(value=A)
    y = K.variable(value=B)

    z = K.dot(x, y)
    # Here you need to use K.eval() instead of z.eval() because this uses the backend session
    K.eval(z)
    z = K.get_value(z)[0]
    return "hello {}, here's your random tensor  {}".format(input, str(z))
