// Package main is the entry point.
package main

// line comment

import "fmt"

/* block comment */

func main() {
    s := "/* not a block */ // not a line"
    // real comment
    fmt.Println(s) // trailing

    /*
     * multi-line
     */
}
