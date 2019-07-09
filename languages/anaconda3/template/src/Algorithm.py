import Algorithmia
from Algorithmia import Handler

def apply(input):
    print("hello " + input)


algo = Handler(apply)
algo.serve()
