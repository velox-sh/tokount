// 12 lines 8 code 3 comments 1 blank
package com.example

/* Outer /* nested */ comment */
fun main() {
    // Print greeting
    val name = "World"
    val msg = """
        Hello, ${name}!
    """.trimIndent()
    println(msg)
}
