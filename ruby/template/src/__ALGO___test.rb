require 'minitest/autorun'
require_relative '__ALGO__'

class AlgorithmTest < Minitest::Test
  def test___ALGO__
    assert_equal apply('Jane'), 'Hello Jane'
  end
end
