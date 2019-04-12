import os
import six
import base64
import json
import traceback
import sys

FIFO_PATH = '/tmp/algoout'


def pipe_startup():

    try:
        os.mkfifo(FIFO_PATH)
    except:
        pass
    print('PIPE_INIT_COMPLETE')
    sys.stdout.flush()


def is_binary(arg):
    if six.PY3:
        return isinstance(arg, base64.bytes_types)
    return isinstance(arg, bytearray)


def get_response(apply_method, request, context=None):
    try:
        result = call_algorithm(apply_method, request, context)
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
        if hasattr(e, 'error_type'):
            error_type = e.error_type
        else:
            error_type = 'AlgorithmError'
        response_string = json.dumps({
            'error': {
                'message': str(e),
                'stacktrace': traceback.format_exc(),
                'error_type': error_type
            }
        })

    return response_string


def wrap_binary_data(data):
    if six.PY3:
        return bytes(data)
    return bytearray(data)


def call_algorithm(apply_method, request, context=None):
    if request['content_type'] in ['text', 'json']:
        data = request['data']
    elif request['content_type'] == 'binary':
        data = wrap_binary_data(base64.b64decode(request['data']))
    else:
        raise Exception("Invalid content_type: {}".format(request['content_type']))
    if context:
        return apply_method(data, context)
    else:
        return apply_method(data)


def write_to_fifo(response):
    # Flush stdout before writing back response
    sys.stdout.flush()

    with open(FIFO_PATH, 'w') as f:
        f.write(response)
        f.write('\n')
