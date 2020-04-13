#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[cfg(test)]
mod write_with_properties_test {
    // use crate::error::Error;
    use crate::procedure::basic_impl::basic_procedure::BasicProcedure;
    use crate::procedure::basic_impl::basic_properties::BasicProperties;
    use crate::procedure::procedure::Procedure;
    use crate::test::test_data::test_data;
    use std::collections::HashMap;
    // use std::result;

    #[test]
    fn should_write_with_properties() {
        let mut config = HashMap::new();
        config.insert(
            String::from("file"),
            String::from("output_with_properties.txt"),
        );
        config.insert(String::from("graph-format"), String::from("s6"));
        let proc = BasicProcedure::new_with_config("write_with_properties", config);

        let mut graphs = vec![];
        let properties = BasicProperties { colorable: false };
        graphs.push((test_data::get_petersen_graph(), properties));
        // let pair = ();
        let result = proc.write_with_properties(&mut graphs);
        println!("{:?}", result);
    }
}
