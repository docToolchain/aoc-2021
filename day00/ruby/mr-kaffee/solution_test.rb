require "test/unit"
require_relative "./solution"

#tag::test_class[]
class TestSolution < Test::Unit::TestCase
  def test_say_hello
    assert_equal "Hello, World!", say_hello
    assert_equal "Hello, Eric Wastl!", say_hello("Eric Wastl")
  end
end

#end::test_class[]
