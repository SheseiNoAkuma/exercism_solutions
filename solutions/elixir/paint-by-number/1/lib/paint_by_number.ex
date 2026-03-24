defmodule PaintByNumber do
  def palette_bit_size(color_count) do
    do_palette_bit_size(color_count, 0)
  end

  defp do_palette_bit_size(color_count, test_count) do
    cond do
      :math.pow(2, test_count) < color_count -> do_palette_bit_size(color_count, test_count + 1)
      true -> test_count
    end
  end

  def empty_picture() do
    <<0::0>>
  end

  def test_picture() do
    <<0::2, 1::2, 2::2, 3::2>>
  end

  def prepend_pixel(picture, color_count, pixel_color_index) do
    palette_size = palette_bit_size(color_count)
    <<pixel_color_index::size(palette_size), picture::bitstring>>
  end

  def get_first_pixel(<<>>, _color_count), do: nil
  def get_first_pixel(picture, color_count) do
    palette_size = palette_bit_size(color_count)
    <<first_pixel::size(palette_size), _the_rest::bitstring>> = picture
    first_pixel
  end

  def drop_first_pixel(<<>>, _color_count), do: <<>>
  def drop_first_pixel(picture, color_count) do
    palette_size = palette_bit_size(color_count)
    <<_first_pixel::size(palette_size), the_rest::bitstring>> = picture
    the_rest
  end

  def concat_pictures(picture1, picture2) do
    <<picture1::bitstring, picture2::bitstring>>
  end
end





# You can ignore this file if you're solving this exercise in the web editor,
# or on your own computer if you have Elixir version 1.12 or higher.
# You can check which Elixir version you have by running `elixir -v` in your terminal.
defmodule Math do
  import Bitwise

  # copied from https://github.com/elixir-lang/elixir/blob/v1.12.0/lib/elixir/lib/integer.ex#L103-L114
  def pow(base, exponent) when is_integer(base) and is_integer(exponent) do
    if exponent < 0, do: raise("exponent cannot be negative")
    guarded_pow(base, exponent)
  end

  # https://en.wikipedia.org/wiki/Exponentiation_by_squaring
  defp guarded_pow(_, 0), do: 1
  defp guarded_pow(b, 1), do: b
  defp guarded_pow(b, e) when (e &&& 1) == 0, do: guarded_pow(b * b, e >>> 1)
  defp guarded_pow(b, e), do: b * guarded_pow(b * b, e >>> 1)
end