// This powerful wrapper provides the ability to store a positive integer value.
// TODO: Rewrite it using a generic so that it supports wrapping ANY type.
struct Wrapper<T> {
    value: T,
}

// TODO: Adapt the struct's implementation to be generic over the wrapped value.
impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}
/*
Wrapper<T>: The T here is the generic type that the struct is defined over.
"Hey, I'm about to define some generic methods or associated functions for a type, which will work for any type T"

impl<T>: The T in the impl block is saying "we're implementing methods
that work for any Wrapper<T> where T could be any type."
 -- When you see Wrapper<T>, T here represents the generic type parameter that Wrapper can hold.
It's a way of defining a generic struct or enum that can hold any type.
the <T> after impl tells that
 */
fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
