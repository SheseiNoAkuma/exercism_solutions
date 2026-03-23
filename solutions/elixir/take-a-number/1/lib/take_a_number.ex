defmodule TakeANumber do
  def start() do
    spawn(fn -> loop_process() end)
  end

  defp loop_process(state \\ 0) do
    receive do
      {:report_state, sender} ->
        send(sender, state)
        loop_process(state)

      {:take_a_number, sender} ->
        send(sender, state + 1)
        loop_process(state + 1)

      :stop ->
        :ok

      whatever ->
        loop_process(state)
    end
  end
end
