const Algorithm = require("../src/algorithm.js");
const readline = require('readline');
const fs = require('fs');
const FIFO_PATH = '/tmp/algoout';

function get_response(line) {
    try {
        request = JSON.parse(line);
        result = call_algorithm(request);
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
    } catch (e) {
        response = {
            message: e.toString(),
            stacktrace: e.stack
        }
    }
    return response;
}

function call_algorithm(request) {
    if (typeof request != 'object') {
        throw new Error('Request needs to be an object');
    }

    if (!('data' in request)) {
        throw new Error('data was not in request');
    }

    if ('content_type' in request) {
        if (request['content_type'] === 'text' || request['content_type'] === 'json') {
            data = request['data'];
        } else if (request['content_type'] === 'binary') {
            data = new Buffer(request['data'], 'base64');
        } else {
            throw new Error('Invalid content_type: ' + request['content_type']);
        }
        return Algorithm.apply(data);
    } else {
        throw new Error('content_type was not in the request');
    }
}

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

var alreadyWorking = false;
rl.on('line', (line) => {
    if (!alreadyWorking && line.length > 0) {
        alreadyWorking = true;
        response = get_response(line);
        fd = fs.openSync(FIFO_PATH, 'a');
        fs.writeSync(fd, JSON.stringify(response));
        fs.writeSync(fd, '\n');
        // Flushing here causes errors...
        fs.closeSync(fd);
        rl.write('\n');
        alreadyWorking = false;
    }
});
