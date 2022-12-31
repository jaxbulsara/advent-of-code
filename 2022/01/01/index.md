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
