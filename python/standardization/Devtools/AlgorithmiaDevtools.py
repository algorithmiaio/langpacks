import json
from internal import get_response, \
    write_to_fifo, pipe_startup
from six.moves import input


class AlgorithmHandler:

    def __init__(self, apply=None):
        self._context = dict()
        self._apply = apply
        self._load_func = None
        self._debug_input = None

    def set_apply_function(self, func):
        self._apply = func

    def set_on_load_function(self, func):
        self._load_func = func

    def set_example_input(self, input):
        self._debug_input = input

    def _load(self):
        pipe_startup()
        if self._load_func:
            self._context = self._load_func(self._context)

    def _faas_execute(self):
        while True:
            try:
                line = input()
            except EOFError:
                break
            request_string = json.loads(line)
            response_string = get_response(self._apply, request_string, self._context)
            write_to_fifo(response_string)

    def run(self):
        self._load()
        self._faas_execute()

