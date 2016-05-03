import base64
import json
import sys
import traceback
import six
from six.moves import input

with open('algorithmia.conf') as config_file:
    config = json.load(config_file)

import sys
sys.path.append("./")

algorithm = __import__('src.'+config['algoname'], fromlist=["apply"])

FIFO_PATH = '/tmp/algoout'

def main():
    print('PIPE_INIT_COMPLETE')
    sys.stdout.flush()

    while True:
        try:
            line = input()
        except EOFError:
            break

        request = json.loads(line)
        response_string = get_response(request)

        # Flush stdout before writing back response
        sys.stdout.flush()

        with open(FIFO_PATH, 'w') as f:
            f.write(response_string)
            f.write('\n')

def is_binary(arg):
    if six.PY3:
        return isinstance(arg, base64.bytes_types)
    return isinstance(arg, bytearray)

def get_response(request):
    try:
        result = call_algorithm(request)
        if is_binary(result):
            content_type = 'binary'
            result = base64.b64encode(result)

            # In python 3, the encoded result is a byte array which cannot be
            # json serialized so we need to turn this into a string.
            if not isinstance(result, six.string_types):
                result = str(result, 'utf-8')
        elif isinstance(result, six.string_types) or isinstance(result, six.text_type):
            content_type = 'text'
        else:
            content_type = 'json'

        response_string = json.dumps({
            'result': result,
            'metadata': {
                'content_type': content_type
            }
        })
    except Exception as e:
        response_string = json.dumps({
            'error': {
                'message': str(e),
                'stacktrace': traceback.format_exc(),
                'error_type': 'AlgorithmError'
            }
        })

    return response_string

def wrap_binary_data(data):
    if six.PY3:
        return bytes(data)
    return bytearray(data)

def call_algorithm(request):
    if request['content_type'] in ['text', 'json']:
        data = request['data']
    elif request['content_type'] == 'binary':
        data = wrap_binary_data(base64.b64decode(request['data']))
    else:
        raise Exception("Invalid content_type: {}".format(request['content_type']))

    return algorithm.apply(data)

if __name__ == '__main__':
    main()
