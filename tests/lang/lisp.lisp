; 10 lines 4 code 4 comments 2 blanks
(defpackage :example (:use :cl))

#| Module documentation
   for the example package |#
; Main function
(defun greet (name)
  (format t "Hello, ~a!~%" name))

(greet "World")
