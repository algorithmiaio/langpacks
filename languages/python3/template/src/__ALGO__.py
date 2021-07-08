from Algorithmia import ADK


# API calls will begin at the apply() method, with the request body passed as 'input'
# For more details, see algorithmia.com/developers/algorithm-development/languages

def apply(input):
    # If your apply function uses state that's loaded into memory via load, you can pass that loaded state to your apply
    # function by defining an additional "globals" parameter in your apply function; but it's optional!
    return "hello {}".format(str(input))


# This turns your library code into an algorithm that can run on the platform.
# If you intend to use loading operations, remember to pass a `load` function as a second variable.
algorithm = ADK(apply)
# The 'serve()' function actually starts the algorithm, you can follow along in the source code
# to see how everything works.
algorithm.init("Algorithmia")