defmodule Username do
  def sanitize([]), do: ~c""

  def sanitize([head | tail]) do
    char =
      case head do
        ?ä -> ~c"ae"
        ?ö -> ~c"oe"
        ?ü -> ~c"ue"
        ?ß -> ~c"ss"
        ?_ -> ~c"_"
        c when c in ?a..?z -> [c]
        _ -> ~c""
      end

    char ++ sanitize(tail)
  end
end
