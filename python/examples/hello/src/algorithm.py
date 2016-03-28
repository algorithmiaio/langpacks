import numpy # make sure we can import something that isn't there by default

def apply(input):
    return "James says, 'hello {} - isNan:{}!'".format(input, numpy.isnan(int(input)))
