import Algorithmia
import h2o

h2o.init()


def apply(input):
    """
    This h2o algorithm loads the h2o.init() operation, and then proceeds with normal hello world
    """

    return "hello " + input
