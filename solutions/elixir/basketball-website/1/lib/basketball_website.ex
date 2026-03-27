defmodule BasketballWebsite do
  def extract_from_path(data, path) do
    String.split(path, ".")
    |> do_extract_from_path(data)
  end

  defp do_extract_from_path(_, nil), do: nil
  defp do_extract_from_path([], data), do: data
  defp do_extract_from_path([key | tail], data) do
    do_extract_from_path(tail, data[key])
  end

  def get_in_path(data, path) do
    keys = String.split(path, ".")
    Kernel.get_in(data, keys)
  end
end
