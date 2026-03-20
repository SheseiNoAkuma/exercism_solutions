defmodule NameBadge do
  def print(id, name, department) do
    id_pretty = if id, do: "[#{id}] - ", else: ""
    department_pretty = if department, do: department |> String.upcase(), else: "OWNER"

    id_pretty <> "#{name} - #{department_pretty}"
  end
end
