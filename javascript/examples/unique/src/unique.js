var _ = require('underscore');

exports.apply = function(input, cb) {
    var unique = _.uniq(input);
    cb(null, unique);
};
