# 11 lines 7 code 3 comments 1 blank
defmodule Greeter do
  # Greets a person by name
  def hello(name) do
    greeting = "Hello, #{name}!"
    IO.puts(greeting)
  end

  # Entry point
  def main, do: hello("World")
end
