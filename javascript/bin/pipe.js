const Algorithm = require("../src/algorithm.js");
const readline = require('readline');
const fs = require('fs');
const FIFO_PATH = '/tmp/algoout';

function start_call(line, cb) {
    try {
        request = JSON.parse(line);
        data = get_data(request);

        Algorithm.apply(data, cb);
    } catch (error) {
        cb(error, null);
    }
}

function get_data(request) {
    if (typeof request != 'object') {
        throw new Error('Request needs to be an object');
    }

    if (!('data' in request)) {
        throw new Error('data was not in request');
    }

    if (!('content_type' in request)) {
        throw new Error('content_type was not in the request');
    }

    if (request['content_type'] === 'text' || request['content_type'] === 'json') {
        data = request['data'];
    } else if (request['content_type'] === 'binary') {
        data = new Buffer(request['data'], 'base64');
    } else {
        throw new Error('Invalid content_type: ' + request['content_type']);
    }
    return data;
}

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

var alreadyWorking = false;
function algoCallback(calledAlready, error, result) {
    if (calledAlready[0]) {
        return; // Stop right away if this has been called already
    }
    calledAlready[0] = true;

    if (error) {
        stacktrace = "[None]";
        if (error instanceof Error) {
            stacktrace = error.stack;
        }

        response = {
            error: {
                message: error.toString(),
                stacktrace: stacktrace,
                error_type: 'AlgorithmError'
            }
        }
    } else {
        content_type = 'json';
        if (Buffer.isBuffer(result)) {
            result = result.toString('base64');
            content_type = 'binary';
        }

        response = {
            result: result,
            metadata: {
                content_type: content_type
            }
        };
    }

    fd = fs.openSync(FIFO_PATH, 'a');
    fs.writeSync(fd, JSON.stringify(response));
    fs.writeSync(fd, '\n');
    // Flushing here causes errors...
    fs.closeSync(fd);
    rl.write('\n');
    alreadyWorking = false;
}

rl.on('line', (line) => {
    if (!alreadyWorking && line.length > 0) {
        alreadyWorking = true;
        start_call(line, algoCallback.bind(null /*this*/, [false]));
    }
});
