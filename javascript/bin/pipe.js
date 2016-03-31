const readline = require('readline');


main();

function main() {
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout
    });

    rl.on('line', (line) => {
        request = JSON.parse(line);
        response = get_response(request);

        console.log('You just typed: ' + request);
        rl.write('REPLACE WITH NEW LINE');
    });

    console.log("Starting");
}

function get_response(request) {
    try {
        result = call_algorithm(request);
    } catch (err) {
        console.log(`Got an error ${err}`);
    }
}

function call_algorithm(request) {
    if (typeof request != 'object') {
        throw new Error('Request needs to be an object');
    }

    if ('content_type' in request) {
        if (request['content_type'] === 'text' || request['content_type'] === 'json') {
            data = request['data'];
        } else if (request['content_type'] === 'binary') {
            data = request['data'];
        } else {
            throw new Error('Invalid content_type: ' + request['content_type']);
        }
    } else {
        throw new Error('content_type was not in the request');
    }
}