from Devtools import AlgorithmHandler
import Algorithmia

# API calls will begin at the apply() method, with the request body passed as 'input'
# For more details, see algorithmia.com/developers/algorithm-development/languages
def foo(input, context = None):
    return "hello {}".format(input)

def configure():
    algorithm = AlgorithmHandler(foo)
    return algorithm
