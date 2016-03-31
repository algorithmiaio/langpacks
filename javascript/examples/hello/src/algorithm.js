var _ = require('underscore');

exports.apply = function(input) {
    var sum = _.reduce([1, 2, 3], function(memo, num){ return memo + num; }, 0);
    return "Hello user " + input + " and the sum of the first 3 integers is: " + sum;
};
