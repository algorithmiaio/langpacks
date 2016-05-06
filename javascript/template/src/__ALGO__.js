Algorithmia = require("algorithmia");

exports.apply = function(input, cb) {
    cb(null, "Hello " + input);
};
