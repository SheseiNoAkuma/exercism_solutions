defmodule FileSniffer do
  def type_from_extension("exe"), do: "application/octet-stream"
  def type_from_extension("bmp"), do: "image/bmp"
  def type_from_extension("png"), do: "image/png"
  def type_from_extension("jpg"), do: "image/jpg"
  def type_from_extension("gif"), do: "image/gif"
  def type_from_extension(_), do: nil

  def type_from_binary(<<0x7F, 0x45, 0x4C, 0x46, _the_rest::binary>>), do: "application/octet-stream"
  def type_from_binary(<<0x42, 0x4D, _the_rest::binary>>), do: "image/bmp"
  def type_from_binary(<<0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, _the_rest::binary>>), do: "image/png"
  def type_from_binary(<<0xFF, 0xD8, 0xFF, _the_rest::binary>>), do: "image/jpg"
  def type_from_binary(<<0x47, 0x49, 0x46, _the_rest::binary>>), do: "image/gif"
  def type_from_binary(_), do: nil

  def verify(<<0x7F, 0x45, 0x4C, 0x46, _the_rest::binary>> = binary_file, "exe"), do: {:ok, type_from_binary(binary_file)}
  def verify(<<0x42, 0x4D, _the_rest::binary>> = binary_file, "bmp"), do: {:ok, type_from_binary(binary_file)}
  def verify(<<0xFF, 0xD8, 0xFF, _the_rest::binary>> = binary_file, "jpg"), do: {:ok, type_from_binary(binary_file)}
  def verify(<<0x47, 0x49, 0x46, _the_rest::binary>> = binary_file, "gif"), do: {:ok, type_from_binary(binary_file)}
  def verify(<<0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, _the_rest::binary>> = binary_file, "png") do
    {:ok, type_from_binary(binary_file)}
  end
  def verify(_, _), do: {:error, "Warning, file format and file extension do not match."}
end