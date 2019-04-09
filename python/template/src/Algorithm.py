import AlgorithmiaDevtools
import Algorithmia

# API calls will begin at the apply() method, with the request body passed as 'input'
# For more details, see algorithmia.com/developers/algorithm-development/languages


def apply(input, context=None):
    local_path = context['local_path']
    if 'metrics' in context:
        context['metrics'] += 1
    else:
        context['metrics'] = 1
    return "hello {}, the file {} is installed.".format(input, local_path)


# This is a user defined functor that mutates a 'context' object.
def download_model(context):
    url = "data://zeryx/collection/stream_file_8.mp4"
    client = Algorithmia.client()
    local_path = client.file(url).getFile().name
    print("file downloaded")
    context['local_path'] = local_path
    return context


# Hey this is important don't forget it and don't erase it
def configure():
    algorithm = AlgorithmiaDevtools.AlgorithmHandler()
    algorithm.set_apply_function(apply)
    algorithm.set_on_load_function(download_model)
    return algorithm

