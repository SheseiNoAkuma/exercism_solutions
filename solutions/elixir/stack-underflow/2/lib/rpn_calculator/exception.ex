defmodule RPNCalculator.Exception do
  defmodule DivisionByZeroError do
    defexception message: "division by zero occurred"
  end

  defmodule StackUnderflowError do
    defexception message: "stack underflow occurred"

    @impl true
    def exception(detail) do
      message = "stack underflow occurred"

      case detail do
        [] -> %StackUnderflowError{}
        _ -> %StackUnderflowError{message: message <> ", context: " <> detail}
      end
    end
  end

  def divide([]), do: raise(StackUnderflowError, "when dividing")
  def divide([_]), do: raise(StackUnderflowError, "when dividing")
  def divide([0 | _]), do: raise(DivisionByZeroError)
  def divide([a, b | _]), do: div(b, a)
end
