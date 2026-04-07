defmodule CaptainsLog do
  @planetary_classes ["D", "H", "J", "K", "L", "M", "N", "R", "T", "Y"]

  def random_planet_class() do
    Enum.random(@planetary_classes)
  end

  def random_ship_registry_number() do
    Enum.random(1000..9999)
    |> Integer.to_string()
    |> then(&("NCC-" <> &1))
  end

  def random_stardate() do
    :rand.uniform() + 41000
  end

  def format_stardate(stardate) do
    :erlang.float_to_binary(stardate, decimals: 1)
  end
end
