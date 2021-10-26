#[cfg(test)]
pub mod test_data {
    use crate::graph::edge::EdgeConstructor;
    use crate::graph::graph::{Graph, GraphConstructor};
    use crate::graph::undirected::edge::UndirectedEdge;
    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::matching::perfect_matchings::Matching;

    pub const SNARK_IN_G6_10_PETERSEN: &str = "I?h]@eOWG";
    #[allow(dead_code)]
    pub const SNARK_IN_G6_18: &str = "Q?gY@eOGGC?B_??@g_??DO?O?GW";
    pub const SNARK_IN_G6_20: &str = "Ss??GOGA_I?c????GOQAACGO_P?_K@?S?";
    #[allow(dead_code)]
    pub const SNARK_IN_G6_22: &str = "U?hW@eOGG?GA_A?_g???@?B??@_C?_??S?GO??@W";
    #[allow(dead_code)]
    pub const SNARK_IN_G6_24: &str = "W?gW@eOGG?GA_A??g??C@?@??@_C???OTC????BG?O_??GP";
    #[allow(dead_code)]
    pub const SNARK_IN_G6_26: &str = "Y?hW@eOGG?GA_A?__??CP?@??@_????OP??A??@G?@C??@@???o???D_";
    #[allow(dead_code)]
    pub const SNARK_IN_G6_28: &str =
        "[?GI?a???CCA_?OC__??X_A@??gO??@_CO_???_g_???W??G??????B__?@?A??D";
    #[allow(dead_code)]
    pub const SNARK_IN_G6_30: &str =
        "]C@W@U?GG?G?_A??_?GCP?_O?KG?@????????C?A??@???X?@?O???K_??S???A`K????G_??G";
    pub const SNARK_IN_G6_30_CYC_5: &str =
        "]D?O@S??G??@??B?g??OP_G@????O?C?C_A?@?GG@?A??_`_??_?`??_AO????G@OC?????G?W";
    #[allow(dead_code)]
    pub const SNARK_IN_G6_32: &str =
        "_??W?E??G?GB_AO_g_?CP_??O?G?_???KG?????@??C???EG??G???SAC???O?O@GO??????KGA??_????EC";
    #[allow(dead_code)]
    pub const SNARK_IN_G6_34: &str = "a??A@eOOK??AP??Bo??G??????G_??O@CG????H?G?????O??D????B_?A?_??@PG?A?????_H?G???CO??CG?G?????C@G";
    pub const SNARK_IN_G6_30_GIRTH_6: &str =
        "]D@Q?UGOGA?????Dc???PG???GGC??O???CG??g???E???F_??_@??O??a???CG?????o????w";
    pub const SNARK_IN_G6_36: &str = "c?HI@cO?GC?@_AOCp????_G??C??@???O?O????O??`???H???C???_c?_??g??@??C????C?G??g?????CG??A?G?????w??@_?????g@";
    pub const SNARK_IN_G6_38_1: &str = "e?HI?eO?GC?A_AOCp???X???S???????C??A???O?G????????????D_???O???X???H????IH???????O@C_?G?????G?G??Q???????R?O?_????@C??_";
    pub const SNARK_IN_G6_38_2: &str = "e?HG?eO?G??A_AO?@???W??O??gg???CU??_?CO???g???FG????A????????G??????c???@g???E????@c@?????_??GA??????????FG????G?_???A_";
    pub const SNARK_IN_G6_40: &str = "g?`G?e?WG?D????A?@?????g??W?E???eA??D?????G???G???K???BC?O?????@?O??C?CO?IC????`????????O?????w?O??@????W??????B??????_?G@??G?@?G?A";

    pub const NO_SNARK_IN_G6_18: &str = "Qs?GOGA?OG@?CDGIAS@A_O@@?GG";
    pub const NO_SNARK_IN_G6_112: &str = "~?@os??GO????????????????H??W??C_?B???K???w??????????A???@?????????????G????O???????????????????????????????????????????????@??????@??????_??????O?????A??????G???????C??????A???@???C???C???G???C???C???A???A????_???O???O???A???C???G????C???C????C???@????A????O????O????_???G???@?????_???A????C????O????O????C????_???C????_???O?????A????C????A????O?????@????G?????O????_?????G???@??????A????@??????_???@??????????C@??????????A?A?????????A?O?????????@@??????????GA???????????A@???????????__??????????_G??????????????C?_???????????C@??????????@?C????????????OC??????????@?C????????????c????????????CA????????????A?O??????A???@?????????G???G??????????@??A??????????@???A??????????C????G?????????@???A??????????G????O??????????A???G???????????????????C@??????????????GA??????????????GA??????????????C@??????????????@?O??????????????A?_??????????????_G??????????????C@?????????????W????????????????@G????????????????W????????????????o????A???????????????W?C???????????????W?O???????????????H?G???????????????E?A????????????????Q?_???????????????B?";

    pub const NO_SNARK_IN_S6_50: &str = ":qc@Wo]YJes?[EeD_?W?IDW@AX_}EIFSWGaMVeExxuHaOppXY]QPTsAEULIP`QEKTOPAYI?BAq@pMaVNrApySQkERcO_mgCb^";
    pub const NO_SNARK_IN_S6_112: &str = ":~?@o_GA?`_UCc?_JaOiKBwsMc@EQCpQWDh][EhmeDhiWexeVfXqGFg_^aQEIGgkcaaUHGGkbbAYLIG{kbaeNIwwibQuKHwS]`qQCFw[a`AEDGGWd`aM]HiClgambIX{fhQqcIi?glb}{PJ\\@nSUwOjk}mSAyOxksdq}\\JhcoeBUYKXWrfBJMTk|VsDbPU\\HYtTvSVLLZpc^IR[lKqCe@Vd}AWEE@WeUAWuQBXeeBXub";

    pub const SNARK_IN_S6_10_PETERSEN: &str = ":Ig?SPc_EOrOFCQN";
    #[allow(dead_code)]
    pub const SNARK_IN_S6_76_3TF1_03: &str = ":~?@K_GEA_wCC`WGE_?QGaWiJ`oqIBWcKbxAPchMPDGwS``GUc@MWeXiTExqYFXyZFxc\\gYIbhI?dgqYaHYahiimkiaufJX{jJyckkZIrjbQtkrYvlBaqLjiznJuxNjo~mrz@okNCpWLBPk@EmCPG_CHDaqO|~";

    pub const SNARK_IN_BA_10_PETERSEN: &str =
        "\n1\n10\n4 6 8 \n5 6 9 \n4 7 9 \n5 7 8 \n0 2 5 \n1 3 4 \n0 1 7 \n2 3 6 \n0 3 9 \n1 2 8 \n";

    // pub const SNARK_IN_ADJ_10_PETERSEN: &str = "0000101010\n0000011001\n0000100101\n0000010110\n1010010000\n0101100000\n1100000100\n0011001000\n1001000001\n0110000010\n";

    pub const GG_30_G05_CYC4_G6_100_FILE_PATH: &str =
        "resources/test/Generated_graphs.30.05.sn.cyc4.100.g6";
    pub const GG_30_G05_CYC4_G6_FILE_PATH: &str =
        "resources/test/Generated_graphs.30.05.sn.cyc4.g6";
    pub const GG_30_G05_CYC5_G6_100_FILE_PATH: &str =
        "resources/test/Generated_graphs.30.05.sn.cyc5.100.g6";

    pub fn get_petersen_graph() -> SimpleGraph {
        let mut graph = SimpleGraph::with_capacity(10, 15);
        graph.add_edge(0, 4);
        graph.add_edge(0, 6);
        graph.add_edge(0, 8);
        graph.add_edge(1, 5);
        graph.add_edge(1, 6);
        graph.add_edge(1, 9);
        graph.add_edge(2, 4);
        graph.add_edge(2, 7);
        graph.add_edge(2, 9);
        graph.add_edge(3, 5);
        graph.add_edge(3, 7);
        graph.add_edge(3, 8);
        graph.add_edge(4, 5);
        graph.add_edge(6, 7);
        graph.add_edge(8, 9);
        graph
    }

    pub fn get_colorable_graph_20() -> SimpleGraph {
        let mut graph = SimpleGraph::with_capacity(20, 30);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(0, 5);
        graph.add_edge(1, 2);
        graph.add_edge(1, 7);
        graph.add_edge(2, 3);
        graph.add_edge(2, 9);
        graph.add_edge(3, 4);
        graph.add_edge(3, 11);
        graph.add_edge(4, 13);
        graph.add_edge(5, 6);
        graph.add_edge(5, 14);
        graph.add_edge(6, 7);
        graph.add_edge(6, 16);
        graph.add_edge(7, 8);
        graph.add_edge(8, 9);
        graph.add_edge(8, 17);
        graph.add_edge(9, 10);
        graph.add_edge(10, 11);
        graph.add_edge(10, 18);
        graph.add_edge(11, 12);
        graph.add_edge(12, 13);
        graph.add_edge(12, 19);
        graph.add_edge(13, 14);
        graph.add_edge(14, 15);
        graph.add_edge(15, 16);
        graph.add_edge(15, 19);
        graph.add_edge(16, 17);
        graph.add_edge(17, 18);
        graph.add_edge(18, 19);

        graph
    }

    pub fn get_falcon_graph() -> SimpleGraph {
        let graph = G6Reader::read_graph(SNARK_IN_G6_36_STABLE_RES_3).unwrap();
        graph
    }

    pub const SNARK_IN_G6_26_CRITICAL_1: &str =
        "Y?gY@eOGGC?B???@__??D??@??k?????C??@??aG?O_??GHO??G??A__";
    pub const SNARK_IN_G6_26_CRITICAL_2: &str =
        "Y?gY@eOGGC?B????__??D??@??k??O??D??@??aG?O???GH?@O???CA_";

    pub const SNARK_IN_G6_26_SCOCRITICAL_1: &str =
        "Y?HI@eO?????_?OCa_?CP_???SG@_??SCG???@?H?G???@C?A?C???B_";
    pub const SNARK_IN_G6_26_SCOCRITICAL_2: &str =
        "Y??Y?EO??CCAP??Bo@A?`A???_KO??a?C_????@H??O?A?A?O?G???B_";

    pub const SNARK_IN_G6_34_STABLE_1: &str =
        "a?gW@eOGG?GA_??_g_?????C?A?C???O???I??@W??W???XO?O??AC?_?_????_??A?????_?????@O????k????o????BG";
    pub const SNARK_IN_G6_34_STABLE_2: &str =
        "aCGG??AGKOG@??C?o?Oa?@?O??y????_DA???@?OG?C???FGO????_?_O??????A?O????_??????@O????k????o????BG";

    pub const SNARK_IN_G6_30_ACRITICAL_1: &str =
        "]D@W@UGGG?GA_A?__???@?A??@_?@??G@?????A???D???J???o???K_OA????O@?AC????C?W";
    pub const SNARK_IN_G6_30_ACRITICAL_2: &str =
        "]C?G?SGG??G@_AO_g_?CP_??C?@_????[O?C?GAGA????_@G????G???A??_???F?C??O???IG";

    pub const SNARK_IN_G6_36_STABLE_34_IER: &str =
        "c?gW@eOGG??A????G???O??g?@W?E??@c????__C??@?@?@?_???A?G_??E????L??????D??A?????G???T????G?@???G????W?????R";

    // falcon graph - 36 vertices and resistance 3
    pub const SNARK_IN_G6_36_STABLE_RES_3: &str =
        "c?gW@eOGG??A????G??C???g?@W?E??@c????A_C??@?@?@?_???A?G_??E????L??????D??A?????G???T????G?@???G????W?????R";

    // pub const SNARK_IN_G6_76: &str =
    //     "~?@KhDGHEG?G?_@?@?O_@G?c??C??G??G??C??@??@G?@@?O?P??AO????_???G???@???CC????G???@G????C????P????P??????_????@?????@??????_????CG?????H?????AO??????G??????G??????C??????@???????G??????C_?????AA?????O?P??????@G????????G???????@????????C???????GG????????G????????c????????@????????AG???????@C?????????@?????????@??????????_?????????G?????????`??????????c?????????C_??????????G??????????C??????????@???????????G???????????a??????????H??????????AA?????????G?Go??????????Q??G???C???A??";

    pub fn first_odd_component() -> Vec<UndirectedEdge> {
        let mut edges = vec![];
        edges.push(UndirectedEdge::new(0, 1));
        edges.push(UndirectedEdge::new(0, 2));
        edges.push(UndirectedEdge::new(1, 4));
        edges.push(UndirectedEdge::new(2, 3));
        edges.push(UndirectedEdge::new(2, 4));
        edges.push(UndirectedEdge::new(3, 4));
        edges
    }

    pub fn second_odd_component() -> Vec<UndirectedEdge> {
        let mut edges = vec![];
        edges.push(UndirectedEdge::new(15, 16));
        edges.push(UndirectedEdge::new(15, 17));
        edges.push(UndirectedEdge::new(16, 17));
        edges
    }

    pub fn first_even_component() -> Vec<UndirectedEdge> {
        let mut edges = vec![];
        edges.push(UndirectedEdge::new(5, 6));
        edges.push(UndirectedEdge::new(5, 7));
        edges.push(UndirectedEdge::new(5, 10));
        edges.push(UndirectedEdge::new(6, 8));
        edges.push(UndirectedEdge::new(6, 9));
        edges.push(UndirectedEdge::new(7, 8));
        edges.push(UndirectedEdge::new(7, 9));
        edges.push(UndirectedEdge::new(8, 10));
        edges.push(UndirectedEdge::new(9, 10));
        edges
    }

    pub fn second_even_component() -> Vec<UndirectedEdge> {
        let mut edges = vec![];
        edges.push(UndirectedEdge::new(11, 12));
        edges.push(UndirectedEdge::new(11, 13));
        edges.push(UndirectedEdge::new(11, 14));
        edges.push(UndirectedEdge::new(12, 13));
        edges.push(UndirectedEdge::new(12, 14));
        edges.push(UndirectedEdge::new(13, 14));
        edges
    }

    pub fn third_even_component_petersen() -> Vec<UndirectedEdge> {
        let mut edges = vec![];
        edges.push(UndirectedEdge::new(18, 22));
        edges.push(UndirectedEdge::new(18, 24));
        edges.push(UndirectedEdge::new(18, 26));
        edges.push(UndirectedEdge::new(19, 23));
        edges.push(UndirectedEdge::new(19, 24));
        edges.push(UndirectedEdge::new(19, 27));
        edges.push(UndirectedEdge::new(20, 22));
        edges.push(UndirectedEdge::new(20, 25));
        edges.push(UndirectedEdge::new(20, 27));
        edges.push(UndirectedEdge::new(21, 23));
        edges.push(UndirectedEdge::new(21, 25));
        edges.push(UndirectedEdge::new(21, 26));
        edges.push(UndirectedEdge::new(22, 23));
        edges.push(UndirectedEdge::new(24, 25));
        edges.push(UndirectedEdge::new(26, 27));
        edges
    }

    pub fn petersens_matchings() -> Vec<Matching> {
        let mut matchings = vec![];
        let mut first_matching = Matching::new();
        first_matching.edges.push(UndirectedEdge::new(0, 6));
        first_matching.edges.push(UndirectedEdge::new(8, 9));
        first_matching.edges.push(UndirectedEdge::new(3, 7));
        first_matching.edges.push(UndirectedEdge::new(1, 5));
        first_matching.edges.push(UndirectedEdge::new(2, 4));

        let mut second_matching = Matching::new();
        second_matching.edges.push(UndirectedEdge::new(0, 4));
        second_matching.edges.push(UndirectedEdge::new(3, 8));
        second_matching.edges.push(UndirectedEdge::new(1, 5));
        second_matching.edges.push(UndirectedEdge::new(2, 9));
        second_matching.edges.push(UndirectedEdge::new(6, 7));

        let mut third_matching = Matching::new();
        third_matching.edges.push(UndirectedEdge::new(2, 4));
        third_matching.edges.push(UndirectedEdge::new(0, 8));
        third_matching.edges.push(UndirectedEdge::new(3, 5));
        third_matching.edges.push(UndirectedEdge::new(6, 7));
        third_matching.edges.push(UndirectedEdge::new(1, 9));

        let mut fourth_matching = Matching::new();
        fourth_matching.edges.push(UndirectedEdge::new(0, 4));
        fourth_matching.edges.push(UndirectedEdge::new(3, 5));
        fourth_matching.edges.push(UndirectedEdge::new(1, 6));
        fourth_matching.edges.push(UndirectedEdge::new(2, 7));
        fourth_matching.edges.push(UndirectedEdge::new(8, 9));

        let mut fifth_matching = Matching::new();
        fifth_matching.edges.push(UndirectedEdge::new(0, 8));
        fifth_matching.edges.push(UndirectedEdge::new(4, 5));
        fifth_matching.edges.push(UndirectedEdge::new(1, 6));
        fifth_matching.edges.push(UndirectedEdge::new(2, 9));
        fifth_matching.edges.push(UndirectedEdge::new(3, 7));

        let mut sixth_matching = Matching::new();
        sixth_matching.edges.push(UndirectedEdge::new(0, 6));
        sixth_matching.edges.push(UndirectedEdge::new(4, 5));
        sixth_matching.edges.push(UndirectedEdge::new(3, 8));
        sixth_matching.edges.push(UndirectedEdge::new(1, 9));
        sixth_matching.edges.push(UndirectedEdge::new(2, 7));

        matchings.push(first_matching);
        matchings.push(second_matching);
        matchings.push(third_matching);
        matchings.push(fourth_matching);
        matchings.push(fifth_matching);
        matchings.push(sixth_matching);
        for matching in matchings.iter_mut() {
            matching.edges.sort();
        }
        matchings.sort();
        matchings
    }
}