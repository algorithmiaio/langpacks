# require 'algorithmia'
require_relative 'algorithmiaPipe'

# API calls will begin at the apply() method, with the request body passed as 'input'
# For more details, see algorithmia.com/developers/algorithm-development/languages
def apply(input, context=nil)
  return "Hello " + input
end


def setupBasic()
  algo = AlgorithmPipe.new(method(:apply))
  return algo
end


