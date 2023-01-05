---
blogpost: true
author: Jay Bulsara
category: Programming
tags: advent-of-code, elixir
language: en
---

# 2022, Day 1, Part 1

[View code
here.](https://github.com/jaxbulsara/advent-of-code/tree/main/2022/01/01)

This is my first foray into the world of Advent of Code since a friend of mine
introduced it to me. I started doing a problem that she was stuck on in Python
and I really enjoyed it. But I can do all of these problems in Python and
eventually the marginal enjoyment will decrease.

So I've decided to do all of these problems in Elixir and in Rust because those
are the two languages that I really want to learn right now. I'm not trying to
get on the leaderboards or anything, so going fast isn't a concern for me.

Another thing I want to do with all of these problems is document my thought
process as I solve them. I just re-set-up my blog on Github Pages using Sphinx,
which makes builds and deployments really fast and seamless, so I'm going to be
posting all of these thoughts on my blog at
[jaybulsara.dev](https://jaybulsara.dev/).

## Problem Statement

--- Day 1: Calorie Counting ---

Santa's reindeer typically eat regular reindeer food, but they need a lot of
magical energy to deliver presents on Christmas. For that, their favorite snack
is a special type of star fruit that only grows deep in the jungle. The Elves
have brought you on their annual expedition to the grove where the fruit grows.

To supply enough magical energy, the expedition needs to retrieve a minimum of
fifty stars by December 25th. Although the Elves assure you that the grove has
plenty of fruit, you decide to grab any fruit you see along the way, just in
case.

Collect stars by solving puzzles. Two puzzles will be made available on each
day in the Advent calendar; the second puzzle is unlocked when you complete the
first. Each puzzle grants one star. Good luck!

The jungle must be too overgrown and difficult to navigate in vehicles or
access from the air; the Elves' expedition traditionally goes on foot. As your
boats approach land, the Elves begin taking inventory of their supplies. One
important consideration is food - in particular, the number of Calories each
Elf is carrying (your puzzle input).

The Elves take turns writing down the number of Calories contained by the
various meals, snacks, rations, etc. that they've brought with them, one item
per line. Each Elf separates their own inventory from the previous Elf's
inventory (if any) by a blank line.

For example, suppose the Elves finish writing their items' Calories and end up
with the following list:

```
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
```

This list represents the Calories of the food carried by five Elves:

- The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of
  6000 Calories.

- The second Elf is carrying one food item with 4000 Calories.

- The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000
  Calories.

- The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total
  of 24000 Calories.

- The fifth Elf is carrying one food item with 10000 Calories.

In case the Elves get hungry and need extra snacks, they need to know which
Elf to ask: they'd like to know how many Calories are being carried by the
Elf carrying the most Calories. In the example above, this is 24000
(carried by the fourth Elf).

Find the Elf carrying the most Calories. How many total Calories is that Elf
carrying?

## Composite Analysis

```{note}
If you're not familiar with composite analysis, read [*Reliable Software
through Composite Design* by Glenford J.
Myers](https://archive.org/details/myers-reliable-software-through-composite-design)
```

Looks like we have a list of lists in the input. Step one would be to parse the
input into a list-of-lists data structure. Step two is to add up each sublist
and find the largest sum.

One wrinkle is that we'll likely need to know the ordinal place of largest sum,
so we'll need account for that in our data structure. Assuming the elves are
good at staying in line, that is. Never trust a data structure to truly reflect
the state of the world over time.

I'll start by creating the problem structure:

```{mermaid}
graph TD
    A[Read input file into memory] --> B
    B[Parse input into a list of calorie lists] -->|Most Abstract Input| C
    C[Calculate calorie sum for each elf #] -->|Most Abstract Output| D
    D[Find highest calorie sum] --> E
    E[Print highest calorie sum]
```

I don't think I even need to do the decomposition since all of these look like
functional strength modules.

## Elixir Solution

[View code here.](
https://github.com/jaxbulsara/advent-of-code/tree/main/2022/01/01/elixir-advent
)

I have no clear idea on how to approach this since all I've done with elixir is
a few exercises and read about it. Guess the best thing to do is take it step
by step. Lucky for me, I have the steps listed out above.

I guess the best way to go about this is to use `mix new` to set up a new
project.

Now to write each module.

### A. Read input file into memory

I'll just use the builtin
[`File.read/1`](https://hexdocs.pm/elixir/File.html#read/1) to read the input.

```elixir
iex(1)> File.read "input.txt"
{:ok, "9195\n5496\n2732\n8364\n3703\n3199\n ... " <> ...}
```

### B. Parse input into a list of calorie lists

Originally, I was thinking of using a `for` loop, but as I got into it, it was
getting kind of unwieldy even for something really simple like this. Then I
remembered that that's not what you do in functional languages if you can help
it.

So really, this step will need to be performed recursively and in two steps:

```{mermaid}
graph TD
  A --> B
  A[Transform input into a list of lines]
  B[Parse input list into a list of calorie lists]
```

For the first step,
[`String.split/3`](https://hexdocs.pm/elixir/String.html#split/3) will do:

```elixir
iex(3)> String.split "123\n234\n345\n", "\n"
["123", "234", "345", ""]
```

Then, I'll need to write a recursive function to transform that list of strings
into a list of calorie lists. I'll need to use one builtin function,
[`String.to_integer/1`](https://hexdocs.pm/elixir/String.html#to_integer/1)

```elixir
iex(4)> String.to_integer "123"
123
```

I'll write a test for my function, `create_calorie_lists/1`, which takes the
list from the previous step. I'll assume that the first and last lines are
always populated:

```{code-block} elixir
:linenos:
:caption: test/elixir_advent_test.exs
test "parses input list of strings into a list of calorie lists" do
  input = ["1", "2", "3", "", "3", "4", "5"]

  expected = [[1, 2, 3], [3, 4, 5]]
  assert ElixirAdvent.create_calorie_lists(input) == expected
end
```

First test run got an error: `** (UndefinedFunctionError) function
ElixirAdvent.create_calorie_lists/1 is undefined or private`. I need to create
the function definition in my module:

```{code-block} elixir
:linenos:
:caption: lib/elixir_advent.ex
defmodule ElixirAdvent do
  def create_calorie_lists(input) do

  end
end
```

Now I can start writing the actual function. The first definition seems to be
just the entry point, because I will eventually need a second argument that
holds the calorie lists. Something like this:

```{code-block} elixir
:linenos:
:caption: lib/elixir_advent.ex
:emphasize-lines: 2, 5-7
  def create_calorie_lists(input) do
    _create_calorie_lists(input, [])
  end

  defp _create_calorie_lists([line | input], []) do

  end
```

Here, I need to start defining each function clause by its input match. I'll
just list out my cases:

- `input`: Start the calorie list creation process with an empty list.
- `[line | input], []`: Create a new sub-list with `line` and add it to the
    calorie list.
- `["" | input], [calorie_group | calorie_tail]`: Create a new empty calorie
    group and join it to the calorie list.
- `[line | input], [calorie_group | calorie_tail]`: Add `line` to
    `calorie_group`
- `[] | calorie_list`: When the input is depleted, return the calorie list.

And I get the following code:

```{code-block} elixir
:linenos:
:caption: lib/elixir_advent.ex
:emphasize-lines: 7, 10-28
defmodule ElixirAdvent do
  def create_calorie_lists(input) do
    _create_calorie_lists(input, [])
  end

  defp _create_calorie_lists([line | input], []) do
    _create_calorie_lists(input, [[_to_calories(line)]])
  end

  defp _create_calorie_lists(["" | input], calorie_list) do
    _create_calorie_lists(input, [[] | calorie_list])
  end

  defp _create_calorie_lists([line | input], [calorie_group | calorie_tail]) do
    _create_calorie_lists(
      input,
      [[_to_calories(line) | calorie_group] | calorie_tail]
    )
  end

  defp _create_calorie_lists([], calorie_list) do
    calorie_list
  end

  defp _to_calories(line) do
    String.to_integer(line)
  end
end
```

Which surprisingly fails, but because the calorie lists are in the reverse
order! I didn't consider that appending heads would result in reversed lists.

```
    code:  assert ElixirAdvent.create_calorie_lists(input) == expected
    left:  [[5, 4, 3], [3, 2, 1]]
    right: [[1, 2, 3], [3, 4, 5]]
    stacktrace:
      test/elixir_advent_test.exs:9: (test)
```

I'll try [`Enum.reverse/1`](https://hexdocs.pm/elixir/Enum.html#reverse/1) to
get the lists in the correct order. Whenever a group is complete, I'll have to
reverse it. And then I'll have to reverse the whole list at the end:

```{code-block} elixir
:linenos:
:caption: lib/elixir_advent.ex
:emphasize-lines: 11-13, 24
defmodule ElixirAdvent do

  def create_calorie_lists(input) do
    _create_calorie_lists(input, [])
  end

  defp _create_calorie_lists([line | input], []) do
    _create_calorie_lists(input, [[_to_calories(line)]])
  end

  defp _create_calorie_lists(["" | input], [calorie_group | calorie_tail]) do
    calorie_group = Enum.reverse(calorie_group)
    _create_calorie_lists(input, [[] | [calorie_group | calorie_tail]])
  end

  defp _create_calorie_lists([line | input], [calorie_group | calorie_tail]) do
    _create_calorie_lists(
      input,
      [[_to_calories(line) | calorie_group] | calorie_tail]
    )
  end

  defp _create_calorie_lists([], calorie_list) do
    Enum.reverse(calorie_list)
  end

  defp _to_calories(line) do
    String.to_integer(line)
  end
end
```

And I get a new error, where the last group remains unreversed:

```
     code:  assert ElixirAdvent.create_calorie_lists(input) == expected
     left:  [[1, 2, 3], [5, 4, 3]]
     right: [[1, 2, 3], [3, 4, 5]]
     stacktrace:
       test/elixir_advent_test.exs:9: (test)

```

Looks like in the last step, I need to reverse the head and then the whole
list:

```{code-block} elixir
:linenos:
:caption: lib/elixir_advent.ex
:emphasize-lines: 22-24
defmodule ElixirAdvent do
  def create_calorie_lists(input) do
    _create_calorie_lists(input, [])
  end

  defp _create_calorie_lists([line | input], []) do
    _create_calorie_lists(input, [[_to_calories(line)]])
  end

  defp _create_calorie_lists(["" | input], [calorie_group | calorie_tail]) do
    calorie_group = Enum.reverse(calorie_group)
    _create_calorie_lists(input, [[] | [calorie_group | calorie_tail]])
  end

  defp _create_calorie_lists([line | input], [calorie_group | calorie_tail]) do
    _create_calorie_lists(
      input,
      [[_to_calories(line) | calorie_group] | calorie_tail]
    )
  end

  defp _create_calorie_lists([], [calorie_group | calorie_tail]) do
    calorie_group = Enum.reverse(calorie_group)
    Enum.reverse([calorie_group | calorie_tail])
  end

  defp _to_calories(line) do
    String.to_integer(line)
  end
end
```

Now my test is passing, so I can refactor the code a little:

```{code-block} elixir
:linenos:
:caption: lib/elixir_advent.ex
:emphasize-lines: 3-5, 9-12, 16-20, 24-27, 31-34
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
end
```

### C. Calculate calorie sum for each elf #

This should be a bit easier. All I need to do is iterate through the list and
add up each group. Then remember to reverse the list at the end.

I'll write my test for `sum_calorie_groups`:

```{code-block} elixir
:linenos:
:caption: test/elixir_advent_test.exs
defmodule ElixirAdventTest do
  use ExUnit.Case
  doctest ElixirAdvent

  ...

  test "sums each calorie group" do
    input = [[1, 2, 3], [3, 4, 5]]

    expected = [6, 12]
    assert ElixirAdvent.sum_calorie_groups(input) == expected
  end
end
```

Then I'll write the function. Here I wrote each case in order, starting from
the first case, with an empty sums list, through to the end, where I need to
reverse the final list.

```{code-block} elixir
:linenos:
:caption: lib/elixir_advent.ex
defmodule ElixirAdvent do
  ...

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
```

Which passes easily. Though it looks like so much code for something as simple
as iterating through a list. But it is fun to write these things explicitly
step-by-step.

### D. Find highest calorie sum

[`Enum.max/3`](https://hexdocs.pm/elixir/Enum.html#max/3) will do the trick:

```elixir
iex(1)> Enum.max([7, 3, 5])
7
```

### E. Print highest calorie sum

Lastly, [`IO.puts/2`](https://hexdocs.pm/elixir/IO.html#puts/2):

```elixir
iex(2)> IO.puts(7)
7
:ok
```

### Writing the main function

Now that we have all of the individual functions we need, we can write the main
function, which will just be a pipe through each one.

```{code-block} elixir
:linenos:
:caption: lib/elixir_advent.ex
defmodule ElixirAdvent do
  def solve(filename) do
    {:ok, input} = File.read(filename)

    input
    |> String.split("\n")
    |> create_calorie_lists()
    |> sum_calorie_groups()
    |> Enum.max()
    |> IO.puts()
  end

  ...
end
```

To run the script, I'll enter `iex` with `iex -S mix` to load the project.

```elixir
ex(4)> recompile
Compiling 1 file (.ex)
:ok
iex(5)> ElixirAdvent.solve("input.txt")
69281
:ok
```

And the answer, for my input, is `69281`!

### Afterthoughts

After browsing through [`Enum`](https://hexdocs.pm/elixir/Enum.html#content), I
noticed [`Enum.map/2`](https://hexdocs.pm/elixir/Enum.html#map/2) which I can
use to replace my `sum_calorie_groups` function:


```{code-block} elixir
:linenos:
:caption: lib/elixir_advent.ex
:emphasize-lines: 8
defmodule ElixirAdvent do
  def solve(filename) do
    {:ok, input} = File.read(filename)

    input
    |> String.split("\n")
    |> create_calorie_lists()
    |> Enum.map(fn group -> Enum.sum(group) end)
    |> Enum.max()
    |> IO.puts()
  end

  ...
end
```
