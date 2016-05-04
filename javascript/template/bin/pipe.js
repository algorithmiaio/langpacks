const readline = require('readline');
const fs = require('fs');
const config = JSON.parse(fs.readFileSync(__dirname+'/../algorithmia.conf', 'utf8'));
const Algorithm = require('../src/'+config.algoname+'.js');
const FIFO_PATH = '/tmp/algoout';

console.log("PIPE_INIT_COMPLETE");

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
            content_type = 'binary';
            result = result.toString('base64');
        } else if (typeof result === 'string') {
            content_type = 'text';
        }

        response = {
            result: result,
            metadata: {
                content_type: content_type
            }
        };
    }

    var jsonStringifiedResponse = null;
    try {
        jsonStringifiedResponse = JSON.stringify(response);
    } catch (error) {
        jsonStringifiedResponse = JSON.stringify({
            error: {
                message: "Cannot json encode result of type: " + typeof(result),
                stacktrace: error.stack,
                error_type: 'AlgorithmError'
            }
        });
    }

    fd = fs.openSync(FIFO_PATH, 'a');
    fs.writeSync(fd, jsonStringifiedResponse);
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
