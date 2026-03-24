(defpackage :example (:use :cl))

#| Module documentation
   for the example package |#
; Main function
(defun greet (name)
  (format t "Hello, ~a!~%" name))

(greet "World")
