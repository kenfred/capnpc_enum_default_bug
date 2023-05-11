pub mod example_capnp {
    include!(concat!(env!("OUT_DIR"), "/example_capnp.rs"));
}

#[cfg(test)]
mod tests {
    use crate::example_capnp::example;

    // This test will get the default value, then set the value with that default,
    // then get it again and compare that the values are equal.
    #[test]
    fn test_encoding_failure_with_default() {
        let mut message = ::capnp::message::Builder::new_default();
        let mut example = message.init_root::<example::Builder>();

        // Default is encoded as 0 (as always)
        // Schema default is "two" = 2, so we expect the mask = 2

        // Get value, decodes with an xor
        // value = 0^2 = 2
        let default_value = example.reborrow_as_reader().get_value().unwrap();

        // Set value to 2
        // What should happen: encodes with an xor: 2^2 = 0
        // What happens: encodes with direct store: 2
        example.set_value(default_value);

        // Get value again, decodes with xor
        // value = 2^2 = 0 
        assert_eq!(default_value, example.reborrow_as_reader().get_value().unwrap());
    }

    // This test will set an enum value, then get it back to make
    // sure it was encoded correctly.
    #[test]
    fn test_encoding_failure_with_other_value() {
        let mut message = ::capnp::message::Builder::new_default();
        let mut example = message.init_root::<example::Builder>();

        // Default is encoded as 0 (as always)
        // Schema default is "two" = 2, so we expect the mask = 2

        // Set value to "One" (1)
        // What should happen: encodes with an xor: 1^2 = 3
        // What happens: encodes with direct store: 1
        example.set_value(example::Variants::One);

        // Get value again, decodes with xor
        // value = 1^2 = 3 (which is invalid) 
        assert_eq!(example::Variants::One, example.reborrow_as_reader().get_value().unwrap());
    }
}
