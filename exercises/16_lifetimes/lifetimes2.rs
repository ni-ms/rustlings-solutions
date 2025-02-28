// Don't change this function.
// Remember that the generic lifetime `'a` will get the concrete lifetime that is
// equal to the smaller of the lifetimes of `x` and `y`.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: Fix the compiler error by moving one line.

    let string1 = String::from("long string is long");
    // Solution1: You can move `strings2` out of the inner block so that it is
    // not dropped before the print statement.
    let string2 = String::from("xyz");

    let result;
    {

        result = longest(&string1, &string2);
    }
    // Solution2: You can move the print statement into the inner block so
    // that it is executed before `string2` is dropped.
    println!("The longest string is '{result}'");
}
