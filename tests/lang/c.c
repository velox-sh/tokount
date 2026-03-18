/* block comment */

// line comment

#include <stdio.h>

int main(void) {
    char *s = "/* not a comment */ // also not";
    /* multi-line
       block comment */
    int x = 5; // trailing

    return 0;
}
