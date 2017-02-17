Algorithmia = require("algorithmia");

/**
 * API calls will begin at the apply() method, with the request body passed as 'input'
 * For more details, see algorithmia.com/developers/algorithm-development/languages
 */
exports.apply = function(input, cb) {
    cb(null, "Hello " + input);
};
