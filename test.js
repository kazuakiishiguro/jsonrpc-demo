#!/usr/bin/env node

var request = require('request');

var headers = {
    'content-type': 'application/json'
};

var dataString = '{"jsonrpc": "2.0", "method": "sum", "params": [10, 2], "id":1 }';

var options = {
    url: 'http://127.0.0.1:3030/',
    method: 'POST',
    headers: headers,
    body: dataString
};

function callback(error, response, body) {
    if (!error && response.statusCode == 200) {
        console.log(body);
    }
}

request(options, callback);
