defmodule ElixirAdvent do
  def create_calorie_lists(input) do
    calorie_list = []

    _create_calorie_lists(input, calorie_list)
  end

  defp _create_calorie_lists([line | input], []) do
    calorie_group = [_to_calories(line)]
    calorie_list = [calorie_group]

    _create_calorie_lists(input, calorie_list)
  end

  defp _create_calorie_lists(["" | input], [calorie_group | calorie_tail]) do
    calorie_group = Enum.reverse(calorie_group)
    calorie_list = [calorie_group | calorie_tail]
    calorie_list = [[] | calorie_list]

    _create_calorie_lists(input, calorie_list)
  end

  defp _create_calorie_lists([line | input], [calorie_group | calorie_tail]) do
    calorie_group = [_to_calories(line) | calorie_group]
    calorie_list = [calorie_group | calorie_tail]

    _create_calorie_lists(input, calorie_list)
  end

  defp _create_calorie_lists([], [calorie_group | calorie_tail]) do
    calorie_group = Enum.reverse(calorie_group)
    calorie_list = Enum.reverse([calorie_group | calorie_tail])

    calorie_list
  end

  defp _to_calories(line) do
    String.to_integer(line)
  end

  def sum_calorie_groups(input) do
    calorie_sums = []

    _sum_calorie_groups(input, calorie_sums)
  end

  def _sum_calorie_groups([calorie_group | calorie_tail], []) do
    sum = Enum.sum(calorie_group)
    calorie_sums = [sum]

    _sum_calorie_groups(calorie_tail, calorie_sums)
  end

  def _sum_calorie_groups([calorie_group | calorie_tail], calorie_sums) do
    sum = Enum.sum(calorie_group)
    calorie_sums = [sum | calorie_sums]

    _sum_calorie_groups(calorie_tail, calorie_sums)
  end

  def _sum_calorie_groups([], calorie_sums) do
    calorie_sums = Enum.reverse(calorie_sums)

    calorie_sums
  end
end
