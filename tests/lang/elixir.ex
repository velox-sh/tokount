defmodule Greeter do
  # Greets a person by name
  def hello(name) do
    greeting = "Hello, #{name}!"
    IO.puts(greeting)
  end

  # Entry point
  def main, do: hello("World")
end
