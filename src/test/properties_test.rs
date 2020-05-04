#[cfg(test)]
pub mod properties_tests {
    use crate::service::chromatic_properties::critical_prop::CriticalProperties;
    use crate::test::test_data::test_data;

    #[test]
    fn test() {
        let graph = test_data::get_colorable_graph_20();
        let mut critical_properties = CriticalProperties::of_graph(&graph);

        let prop = critical_properties.is_edge_subcritical();
        println!("prop: {}", prop);
    }
}
