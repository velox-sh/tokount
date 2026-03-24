fun single-quote():
  doc: "this is a documentation string"
  'foo'
end

#|
  Hello, this is a multiline message
|#

# This is a line message

fun double-quotes():
  "bar"
end

nested = #|
  doesn't start yet
  or yet
|#
"nested"
