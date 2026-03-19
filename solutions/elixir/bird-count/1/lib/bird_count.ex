defmodule BirdCount do
  def today([head | _tail]), do: head
  def today([]), do: nil

  def increment_day_count([head | tail]), do: [head + 1 | tail]
  def increment_day_count([]), do: [1]

  def has_day_without_birds?([head | _tail]) when head == 0, do: true
  def has_day_without_birds?([_head | tail]), do: has_day_without_birds?(tail)
  def has_day_without_birds?([]), do: false

  def total([head | tail]), do: head + total(tail)
  def total([]), do: 0

  def busy_days([head | tail]) when head >= 5, do: 1 + busy_days(tail)
  def busy_days([_head | tail]), do: busy_days(tail)
  def busy_days([]), do: 0
end
