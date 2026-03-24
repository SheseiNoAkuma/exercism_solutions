defmodule DNA do
  @bases %{?A => 0b0001, ?C => 0b0010, ?G => 0b0100, ?T => 0b1000, ?\s => 0b0000}

  def encode_nucleotide(code_point) do
    Enum.find_value(@bases, fn {key, value} -> key == code_point && value end)
  end

  def decode_nucleotide(encoded_code) do
    Enum.find_value(@bases, fn {key, value} -> value == encoded_code && key end)
  end

  def encode(dna) do
    do_encode(dna, <<>>)
  end

  defp do_encode([], accumulator), do: accumulator

  defp do_encode([head | tail], accumulator),
    do: do_encode(tail, <<accumulator::bitstring, encode_nucleotide(head)::4>>)

  def decode(dna) do
    do_decode(dna, [])
  end

  defp do_decode(<<>>, accumulator), do: reverse(accumulator, [])

  defp do_decode(<<head::4, tail::bitstring>>, accumulator),
    do: do_decode(tail, [decode_nucleotide(head) | accumulator])

  defp reverse([], accumulator), do: accumulator

  defp reverse([head | tail], accumulator),
    do: reverse(tail, [head | accumulator])
end
