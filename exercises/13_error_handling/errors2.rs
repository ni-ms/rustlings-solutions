// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the items. Since
// the player typed in the quantity, we get it as a string. They might have
// typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all. What we want
// to do is: If we call the `total_cost` function on a string that is not a
// number, that function will return a `ParseIntError`. In that case, we want to
// immediately return that error from our function and not try to multiply and
// add.
//
// There are at least two ways to implement this that are both correct. But one
// is a lot shorter!

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO: Handle the error case as described above.
    /*
    The ? operator in Rust is a convenient way to handle errors in functions that return a Result or Option type.
    It simplifies error handling by allowing you to propagate errors without needing to write extensive error-checking code.
    Here’s how it works:
    When you use ? on a Result, if the Result is Ok, it unwraps the value inside Ok and the execution continues.
    If the Result is Err, it returns the error from the function it is used in, effectively stopping execution
    and propagating the error.
    So, in your total_cost function, using item_quantity.parse::<i32>()? means:
    If the parsing succeeds and returns Ok(qty), qty is used in the subsequent calculations.
    If the parsing fails and returns Err(e), the function immediately returns Err(e).
    You can use `?` to propagate the error
     */
    let qty = item_quantity.parse::<i32>()?;


    /*
    You can also use a match expression:
     let qty = match item_quantity.parse::<i32>() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
     */

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}
