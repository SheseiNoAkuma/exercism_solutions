defmodule HighScore do
  def new(), do: %{}

  def add_player(scores, name, score \\ 0) do
    Map.put(scores, name, score)
  end

  def remove_player(scores, name) do
    Map.delete(scores, name)
  end

  def reset_score(scores, name) when is_map_key(scores, name), do: %{scores | name => 0}
  def reset_score(scores, name), do: Map.put(scores, name, 0)

  def update_score(scores, name, score) when is_map_key(scores, name),
    do: %{scores | name => score + scores[name]}

  def update_score(scores, name, score), do: Map.put(scores, name, score)

  def get_players(scores), do: Map.keys(scores)
end
