// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
        //first.to_uppercase().map(|c| c.to_string()).collect::<String>() + chars.as_str()
    }
}
// TODO: What does collect do?
// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // ???
    // Or use map:  words.iter().map(|&word| capitalize_first(word)).collect()
    let mut cap_words = Vec::new();
    for &word in words
    {
        cap_words.push(capitalize_first(word));
    }
    cap_words
}
// words.iter().map(|word| capitalize_first(word)).collect()
// ALTERNATIVE SOLUTION  ^^^
// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
/*
`capitalize_first`:

The variable `first` is a `char`. It needs to be capitalized and added to the
remaining characters in `chars` in order to return the correct `String`.

The remaining characters in `chars` can be viewed as a string slice using the
`as_str` method.

The documentation for `char` contains many useful methods.
https://doc.rust-lang.org/std/primitive.char.html

Use `char::to_uppercase`. It returns an iterator that can be converted to a
`String`.

`capitalize_words_vector`:

Create an iterator from the slice. Transform the iterated values by applying
the `capitalize_first` function. Remember to `collect` the iterator.

`capitalize_words_string`:

This is surprisingly similar to the previous solution. `collect` is very
powerful and very general. Rust just needs to know the desired type.

 */
fn capitalize_words_string(words: &[&str]) -> String {
    // ???
    words.iter().map(|&word| capitalize_first(word)).collect()
}
// TODO: What does map do?

fn main() {
    // You can optionally experiment here.
    println!("{:?}", capitalize_words_vector(&["hello", "world"]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
