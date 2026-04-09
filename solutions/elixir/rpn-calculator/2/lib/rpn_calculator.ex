defmodule RPNCalculator do
  def calculate!(stack, operation) do
    operation.(stack)
  end

  def calculate(stack, operation) do
    try do
      calculate!(stack, operation)
      |> then(&{:ok, &1})
    rescue
      _ -> :error
    end
  end

  def calculate_verbose(stack, operation) do
    try do
      calculate!(stack, operation)
      |> then(&{:ok, &1})
    rescue
      e -> {:error, e.message}
    end
  end
end
