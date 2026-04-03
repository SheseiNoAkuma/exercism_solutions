# Use the Plot struct as it is provided
defmodule Plot do
  @enforce_keys [:plot_id, :registered_to]
  defstruct [:plot_id, :registered_to]
end

defmodule CommunityGarden do
  @type t :: %__MODULE__{counter: non_neg_integer(), plots: list(Plot.t())}
  defstruct counter: 0, plots: []

  def start(opts \\ []) do
    Agent.start_link(fn -> %CommunityGarden{} end, opts)
  end

  def list_registrations(pid) do
    Agent.get(pid, fn garden -> garden.plots end)
  end

  def register(pid, register_to) do
    Agent.get_and_update(pid, fn garden ->
      plot = %Plot{plot_id: garden.counter + 1, registered_to: register_to}

      new_state = %CommunityGarden{
        garden
        | counter: garden.counter + 1,
          plots: garden.plots ++ [plot]
      }

      {plot, new_state}
    end)
  end

  def release(pid, plot_id) do
    Agent.get_and_update(pid, fn garden ->
      new_state = %CommunityGarden{
        garden
      | counter: garden.counter,
        plots: garden.plots |> Enum.reject(fn plot -> plot.plot_id == plot_id end)
      }

      {:ok, new_state}
    end)
  end

  def get_registration(pid, plot_id) do
    Agent.get(pid, fn garden ->
      garden.plots |> Enum.find({:not_found, "plot is unregistered"}, fn plot -> plot.plot_id == plot_id end)
    end)
  end
end
