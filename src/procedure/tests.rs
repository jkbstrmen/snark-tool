use crate::graph::undirected::simple_graph::graph::SimpleGraph;
use crate::procedure::basic_procedures::chrom_props::chromatic_properties::ChromaticPropsProcedureBuilder;
use crate::procedure::basic_procedures::chrom_props::config::{
    ChromaticPropertiesToCompute, ChromaticPropsProcedureConfig, ParallelizationType,
};
use crate::procedure::basic_procedures::colour::{
    ColourProcedureBuilder, ColourProcedureConfig, ColouriserType,
};
use crate::procedure::basic_procedures::constructions::{
    ConstructionProcedureBuilder, ConstructionProcedureConfig, ConstructionType,
};
use crate::procedure::basic_procedures::counter::{
    CounterProcedureBuilder, CounterProcedureConfig,
};
use crate::procedure::basic_procedures::filter::{FilterProcedureBuilder, FilterProcedureConfig};
use crate::procedure::basic_procedures::read;
use crate::procedure::basic_procedures::read::{ReadProcedureBuilder, ReadProcedureConfig};
use crate::procedure::basic_procedures::write::{WriteProcedureBuilder, WriteProcedureConfig};
use crate::procedure::procedure::{GraphProperties, Procedure};
use crate::procedure::procedure_chain::ProcedureChain;
use crate::tests::test_data::test_data;
use std::collections::HashMap;

#[test]
fn should_create_procedure_chain() {
    let read_config = ReadProcedureConfig::new(
        test_data::GG_30_G05_CYC4_G6_100_FILE_PATH.to_string(),
        read::G6_FORMAT.to_string(),
        None,
    );
    let read = ReadProcedureBuilder::build(read_config);
    let colour_config = ColourProcedureConfig::default();
    let colour = ColourProcedureBuilder::build(colour_config);
    let counter_config = CounterProcedureConfig::new(true);
    let counter = CounterProcedureBuilder::build(counter_config);

    let mut procedures = vec![];
    procedures.push(read);
    procedures.push(colour);
    procedures.push(counter);

    let chain = ProcedureChain::from_procedures(procedures).unwrap();
    let mut graphs_with_properties: Vec<(SimpleGraph, GraphProperties)> = vec![];
    let _result = chain.run(&mut graphs_with_properties);
}

#[test]
fn should_create_procedures() {
    let read_config = ReadProcedureConfig::new(
        test_data::GG_30_G05_CYC4_G6_100_FILE_PATH.to_string(),
        read::G6_FORMAT.to_string(),
        None,
    );
    let read = ReadProcedureBuilder::build(read_config);
    let colour_config = ColourProcedureConfig::default();
    let colour = ColourProcedureBuilder::build(colour_config);
    let counter_config = CounterProcedureConfig::new(true);
    let counter = CounterProcedureBuilder::build(counter_config);
    let construction_config = ConstructionProcedureConfig::new(ConstructionType::IExtension);
    let construction = ConstructionProcedureBuilder::build(construction_config);
    let filter_properties = HashMap::new();
    let filter_config = FilterProcedureConfig::new(filter_properties);
    let filter = FilterProcedureBuilder::build(filter_config);
    let write_config = WriteProcedureConfig::default();
    let write = WriteProcedureBuilder::build(write_config);
    let to_compute = ChromaticPropertiesToCompute::new();
    let chrom_props_config = ChromaticPropsProcedureConfig::new(
        ColouriserType::Dfs,
        ParallelizationType::None,
        to_compute,
        1,
    );
    let chrom_props = ChromaticPropsProcedureBuilder::build(chrom_props_config);

    let mut procedures: Vec<Box<dyn Procedure<SimpleGraph>>> = vec![];
    procedures.push(read);
    procedures.push(colour);
    procedures.push(counter);
    procedures.push(construction);
    procedures.push(filter);
    procedures.push(write);
    procedures.push(chrom_props);
}
