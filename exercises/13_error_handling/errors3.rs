// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?

use std::num::ParseIntError;

// Don't change this function.
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: Fix the compiler error by changing the signature and body of the
// `main` function.
/*
Using Result in main
Error Propagation: By returning a Result in main, we can use the ? operator to propagate errors
up to the top level. If any operation within main that uses ? fails, the error will be returned from main,
terminating the program gracefully and providing an error message.

Unit Type: The () type is known as the "unit type" in Rust.
It represents an empty tuple and is used when there’s no meaningful value to return. Essentially,
it’s a way to signify that the function doesn’t return any data.
 */
fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // Don't change this line.
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {tokens} tokens.");

    }

    /*
    Added this line to return the `Ok` variant of the expected `Result`.
     */
    Ok(())
}
