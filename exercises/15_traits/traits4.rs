trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// TODO: Fix the compiler error by only changing the signature of this function.
/*
Trait Bound: The impl Licensed syntax is a shorthand for specifying a trait bound.
It means that the parameter must be of a type that implements the Licensed trait.
Polymorphism: By using impl Licensed, the function can accept any type that implements
the Licensed trait, allowing for more flexible and reusable code.
Simplified Syntax: The impl Trait syntax is a more concise way to
write generic functions. The equivalent longer form would be:

rust:
fn compare_license_types<T: Licensed, U: Licensed>(software1: T, software2: U) -> bool {
    software1.licensing_info() == software2.licensing_info()
}

Read https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
 */
fn compare_license_types(software1: impl Licensed, software2: impl Licensed) -> bool {
    software1.licensing_info() == software2.licensing_info()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        assert!(compare_license_types(SomeSoftware, OtherSoftware));
    }

    #[test]
    fn compare_license_information_backwards() {
        assert!(compare_license_types(OtherSoftware, SomeSoftware));
    }
}
