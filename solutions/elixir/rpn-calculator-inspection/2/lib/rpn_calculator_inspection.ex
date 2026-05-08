defmodule RPNCalculatorInspection do
  def start_reliability_check(calculator, input) do
    {:ok, pid} =
      Task.start_link(fn ->
        result = calculator.(input)
        send(self(), {:reliability_check_result, input, result})
      end)

    %{pid: pid, input: input}
  end

  def await_reliability_check_result(%{pid: pid, input: input}, results) do
    receive do
      {:EXIT, ^pid, :normal} ->
        Map.put(results, input, :ok)
      {:EXIT, ^pid, _reason} ->
        Map.put(results, input, :error)
    after
      100 ->
        Map.put(results, input, :timeout)
    end
  end

  def reliability_check(calculator, inputs) do
    # Save the original trap_exit flag value
    original_trap_exit = Process.flag(:trap_exit, true)

    try do
      # Start all reliability checks first
      checks = Enum.map(inputs, &start_reliability_check(calculator, &1))

      # Then await results for all of them
      Enum.reduce(checks, %{}, &await_reliability_check_result/2)
    after
      # Restore the original trap_exit flag value
      Process.flag(:trap_exit, original_trap_exit)
    end
  end

  def correctness_check(calculator, inputs) do
    # Start all tasks first
    tasks = Enum.map(inputs, &Task.async(fn -> calculator.(&1) end))

    # Then await all tasks with 100ms timeout
    Enum.map(tasks, &Task.await(&1, 100))
  end
end
