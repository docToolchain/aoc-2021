require 'test/unit'

require_relative './solution'

# tag::test_class[]
class TestSolution < Test::Unit::TestCase
  CONTENT = %(forward 5
down 5
forward 8
up 3
down 8
forward 2).freeze

  def test_calc_position_1
    assert_equal [15, 10], calc_position_1(CONTENT)
  end

  def test_calc_position_2
    assert_equal [15, 60, 10], calc_position_2(CONTENT)
  end
end
# end::test_class[]
