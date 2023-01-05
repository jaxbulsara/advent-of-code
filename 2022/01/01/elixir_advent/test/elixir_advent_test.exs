defmodule ElixirAdventTest do
  use ExUnit.Case
  doctest ElixirAdvent

  test "parses input list of strings into a list of calorie lists" do
    input = ["1", "2", "3", "", "3", "4", "5"]

    expected = [[1, 2, 3], [3, 4, 5]]
    assert ElixirAdvent.create_calorie_lists(input) == expected
  end
end
