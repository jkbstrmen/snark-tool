#[cfg(test)]
pub mod girth_tests {
    use crate::service::property::girth::girth;
    use crate::test::test_data::test_data;

    #[test]
    fn should_have_girth_five() {
        let graph = test_data::get_petersen_graph();
        let girth = girth(&graph);

        assert_eq!(girth, 5);
    }
}
