defmodule ElixirAdventTest.P202201 do
  use ExUnit.Case
  alias ElixirAdvent.P202201, as: ElixirAdvent
  doctest ElixirAdvent

  test "parses input list of strings into a list of calorie lists" do
    input = ["1", "2", "3", "", "3", "4", "5"]

    expected = [[1, 2, 3], [3, 4, 5]]
    assert ElixirAdvent.create_calorie_lists(input) == expected
  end
end
