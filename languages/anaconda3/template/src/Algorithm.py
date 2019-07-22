import Algorithmia

def apply(input):
    return "hello {}".format(str(input))


algo = Algorithmia.handler(apply)
algo.serve()
