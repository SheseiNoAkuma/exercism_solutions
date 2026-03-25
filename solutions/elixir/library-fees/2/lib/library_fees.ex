defmodule LibraryFees do
  def datetime_from_string(string) do
    case NaiveDateTime.from_iso8601(string) do
      {:ok, datetime} -> datetime
      {:error, _} -> nil
    end
  end

  def before_noon?(datetime) do
    datetime
    |> NaiveDateTime.to_time
    |> Kernel.<(~T[12:00:00])
  end

  def return_date(checkout_datetime) do
    days = if before_noon?(checkout_datetime), do: 28, else: 29

    NaiveDateTime.shift(checkout_datetime, day: days)
    |> NaiveDateTime.to_date()
  end

  def days_late(planned_return_date, actual_return_datetime) do
    NaiveDateTime.to_date(actual_return_datetime)
    |> Date.diff(planned_return_date)
    |> max(0)
  end

  def monday?(datetime) do
    NaiveDateTime.to_date(datetime)
    |> Date.day_of_week()
    |> Kernel.==(1)
  end

  def calculate_late_fee(checkout, return, rate) do
    checkout_datetime = datetime_from_string(checkout)
    return_date_time = datetime_from_string(return)

    return_date(checkout_datetime)
    |> days_late(return_date_time)
    |> Kernel.*(rate)
    |> if(monday?(return_date_time), do: &(trunc(&1 / 2)), else: & &1).()
  end
end
