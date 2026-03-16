defmodule KitchenCalculator do
  def get_volume({_, volume}) do
    volume
  end

  def to_milliliter({start_unit, volume}) do
    {:milliliter, milliliter_conversion_rate(start_unit) * volume}
  end

  def from_milliliter({:milliliter, volume}, other_unit) do
    {other_unit, volume / milliliter_conversion_rate(other_unit)}
  end

  def convert({starting_unit, volume}, ending_unit) do
    to_milliliter({starting_unit, volume})
    |> from_milliliter(ending_unit)
  end

  defp milliliter_conversion_rate(:cup), do: 240
  defp milliliter_conversion_rate(:fluid_ounce), do: 30
  defp milliliter_conversion_rate(:teaspoon), do: 5
  defp milliliter_conversion_rate(:tablespoon), do: 15
  defp milliliter_conversion_rate(:milliliter), do: 1
end
