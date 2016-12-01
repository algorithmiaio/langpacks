var __ALGO__ = require('../src/__ALGO__.js');

describe('Algorithm', function() {
  describe('#apply()', function() {
    it('should result in a hello message', function(done) {
      __ALGO__.apply("Jane", function(err, res) {
        if(err) { return done(err); }

        if(res == "Hello Jane") {
          done();
        } else {
          done("Expected 'Hello Jane'. Receieved '" + res + "'.");
        }
			});
    });
  });
});


