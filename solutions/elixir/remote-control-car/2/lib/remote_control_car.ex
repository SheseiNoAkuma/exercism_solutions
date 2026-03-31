defmodule RemoteControlCar do
  @enforce_keys [:nickname]
  defstruct [:nickname, distance_driven_in_meters: 0, battery_percentage: 100]

  def new(nickname \\ "none") do
    %RemoteControlCar{nickname: nickname}
  end

  def display_distance(%RemoteControlCar{distance_driven_in_meters: distance}) do
    distance |> Integer.to_string() |> Kernel.<>(" meters")
  end

  def display_battery(%RemoteControlCar{battery_percentage: battery}) do
    battery
    |> Integer.to_string()
    |> case do
      "0" -> "empty"
      other -> "at #{other}%"
    end
    |> then(&"Battery #{&1}")
  end

  def drive(%RemoteControlCar{battery_percentage: 0} = remote_car), do: remote_car

  def drive(%RemoteControlCar{} = remote_car) do
    %{
      remote_car
      | battery_percentage: remote_car.battery_percentage - 1,
        distance_driven_in_meters: remote_car.distance_driven_in_meters + 20
    }
  end
end
