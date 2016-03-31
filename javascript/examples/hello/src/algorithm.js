var _ = require('underscore');

exports.apply = function(input, cb) {
    var sum = _.reduce([1, 2, 3], function(memo, num){ return memo + num; }, 0);
    cb(null, "1 user " + input + " and the sum of the first 3 integers is: " + sum);
    cb(null, "2 user " + input + " and the sum of the first 3 integers is: " + sum);

};
