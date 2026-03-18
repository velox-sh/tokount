// line comment — this whole line is a comment

fn main() {
    let _raw = r##"/* not a block comment */ // not a line comment"##;
    let _str = "contains /* fake block */ and // fake line";
    // another comment
    let x = 5; // trailing comment does not make this a comment line

    /* block comment
       spanning two lines */
    let _nested = /* inline block */ 42;
}

/*
 * multi-line block comment
 * at top level
 */
fn foo() {}
