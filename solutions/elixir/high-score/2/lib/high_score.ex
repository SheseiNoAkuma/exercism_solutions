defmodule HighScore do
  @default_score 0

  def new(), do: %{}

  def add_player(scores, name, score \\ @default_score) do
    Map.put(scores, name, score)
  end

  def remove_player(scores, name) do
    Map.delete(scores, name)
  end

  def reset_score(scores, name) when is_map_key(scores, name),
    do: %{scores | name => @default_score}

  def reset_score(scores, name), do: Map.put(scores, name, @default_score)

  def update_score(scores, name, score) when is_map_key(scores, name),
    do: %{scores | name => score + scores[name]}

  def update_score(scores, name, score) do
    Map.update(scores, name, score, &(&1 + score))
  end

  def get_players(scores), do: Map.keys(scores)
end
