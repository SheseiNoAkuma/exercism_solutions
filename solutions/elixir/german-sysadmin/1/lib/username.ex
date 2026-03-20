defmodule Username do
  def sanitize([]), do: []

  def sanitize([head | tail]) do
    char =
      case head do
        ?ä -> [?a, ?e]
        ?ö -> [?o, ?e]
        ?ü -> [?u, ?e]
        ?ß -> [?s, ?s]
        ?_ -> [?_]
        c when c in ?a..?z -> [c]
        _ -> []
      end

    char ++ sanitize(tail)
  end
end
