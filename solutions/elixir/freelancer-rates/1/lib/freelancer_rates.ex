defmodule FreelancerRates do
  def daily_rate(hourly_rate), do: hourly_rate * 8.0

  def apply_discount(before_discount, discount) do
    before_discount - (discount / 100.0 * before_discount)
  end

  def monthly_rate(hourly_rate, discount) do
    hourly_rate
    |> apply_discount(discount)
    |> daily_rate()
    |> Kernel.*(22)
    |> Kernel.ceil()
  end

  def days_in_budget(budget, hourly_rate, discount) do
    discounted_daily_rate = 
      hourly_rate
      |> apply_discount(discount)
      |> daily_rate()
    budget
    |> Kernel./(discounted_daily_rate)
    |> Float.floor(1)
  end
end
