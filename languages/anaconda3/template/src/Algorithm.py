import Algorithmia
from Algorithmia import AlgorithmHandler

def apply(input):
    print("hello " + input)



algo = AlgorithmHandler(apply)
algo.serve()