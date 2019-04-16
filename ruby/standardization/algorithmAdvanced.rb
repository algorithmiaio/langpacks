require 'algorithmia'
require_relative 'algorithmiaPipe'

# API calls will begin at the apply() method, with the request body passed as 'input'
# For more details, see algorithmia.com/developers/algorithm-development/languages
def apply(input, context=nil)
    if context == nil or context.key?("local_path")
        return "Hello " + input
    else
        local_path = context["local_path"]
        return "Hello " + input + " your file is located here: " + local_path
    end
  return "Hello " + input
end

def downloadFile()
    api_key = ENV["ALGORITHMIA_API_KEY"]
    client = algorithmia.client(api_key)
    locl_path = client.file('data://demo/collection/somefile.json').getFile().name
    context = {"local_path" => local_path}
    return context
end


def setupAdvanced()
  algo = AlgorithmPipe.new(method(:apply))
  algo.set_on_load_method(method(:downloadFile))
  return algo
end


