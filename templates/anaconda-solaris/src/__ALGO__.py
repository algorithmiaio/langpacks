import Algorithmia
import solaris

# API calls will begin at the apply() method, with the request body passed as 'input'
# For more details, see algorithmia.com/developers/algorithm-development/languages
def apply(input):
    return "hello {} - this algo imports solaris".format(input)
