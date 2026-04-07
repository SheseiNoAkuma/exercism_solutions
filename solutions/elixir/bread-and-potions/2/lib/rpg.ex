defmodule RPG do
  defmodule Character do
    defstruct health: 100, mana: 0
  end

  defmodule LoafOfBread do
    defstruct []
  end

  defmodule ManaPotion do
    defstruct strength: 10
  end

  defmodule Poison do
    defstruct []
  end

  defmodule EmptyBottle do
    defstruct []
  end

  defprotocol Edible do
    def eat(item, character)
  end

  defimpl Edible, for: LoafOfBread do
    def eat(%LoafOfBread{}, %Character{} = c) do
      {nil, %{c | health: c.health + 5}}
    end
  end

  defimpl Edible, for: ManaPotion do
    def eat(%ManaPotion{strength: strength}, %Character{} = c) do
      {%EmptyBottle{}, %{c | mana: c.mana + strength}}
    end
  end

  defimpl Edible, for: Poison do
    def eat(%Poison{}, %Character{} = c) do
      {%EmptyBottle{}, %{c | health: 0}}
    end
  end

end
