from Algorithmia import ADK

# API calls will begin at the apply() method, with the request body passed as 'input'
# For more details, see algorithmia.com/developers/algorithm-development/languages
def apply(input):
    return "hello {}".format(input)


algorithm = ADK(apply)
algorithm.init("world")
