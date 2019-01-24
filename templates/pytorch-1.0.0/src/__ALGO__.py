import Algorithmia
import torch as th


# API calls will begin at the apply() method, with the request body passed as 'input'
# For more details, see algorithmia.com/developers/algorithm-development/languages
def apply(input):
    A = th.randn(input).cuda()
    B = th.randn(input).cuda()
    C = th.dot(A, B)
    random_number = C.cpu().numpy().tolist()
    return "with a random vector shape of {}, here is your random number {}".format(input, str(random_number))


print(apply(56))