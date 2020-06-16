#[cfg(test)]
pub mod test_data {
    use crate::graph::graph::{Graph, GraphConstructor};
    use crate::graph::undirected::simple_graph::SimpleGraph;

    pub const SNARK_IN_G6_10_PETERSEN: &str = "I?h]@eOWG";
    pub const SNARK_IN_G6_20: &str = "Ss??GOGA_I?c????GOQAACGO_P?_K@?S?";
    pub const SNARK_IN_G6_36: &str = "c?HI@cO?GC?@_AOCp????_G??C??@???O?O????O??`???H???C???_c?_??g??@??C????C?G??g?????CG??A?G?????w??@_?????g@";
    pub const SNARK_IN_G6_40: &str = "g?`G?e?WG?D????A?@?????g??W?E???eA??D?????G???G???K???BC?O?????@?O??C?CO?IC????`????????O?????w?O??@????W??????B??????_?G@??G?@?G?A";

    pub const NO_SNARK_IN_G6_18: &str = "Qs?GOGA?OG@?CDGIAS@A_O@@?GG";
    pub const NO_SNARK_IN_G6_112: &str = "~?@os??GO????????????????H??W??C_?B???K???w??????????A???@?????????????G????O???????????????????????????????????????????????@??????@??????_??????O?????A??????G???????C??????A???@???C???C???G???C???C???A???A????_???O???O???A???C???G????C???C????C???@????A????O????O????_???G???@?????_???A????C????O????O????C????_???C????_???O?????A????C????A????O?????@????G?????O????_?????G???@??????A????@??????_???@??????????C@??????????A?A?????????A?O?????????@@??????????GA???????????A@???????????__??????????_G??????????????C?_???????????C@??????????@?C????????????OC??????????@?C????????????c????????????CA????????????A?O??????A???@?????????G???G??????????@??A??????????@???A??????????C????G?????????@???A??????????G????O??????????A???G???????????????????C@??????????????GA??????????????GA??????????????C@??????????????@?O??????????????A?_??????????????_G??????????????C@?????????????W????????????????@G????????????????W????????????????o????A???????????????W?C???????????????W?O???????????????H?G???????????????E?A????????????????Q?_???????????????B?";

    pub const NO_SNARK_IN_S6_112: &str = ":~?@o_GA?`_UCc?_JaOiKBwsMc@EQCpQWDh][EhmeDhiWexeVfXqGFg_^aQEIGgkcaaUHGGkbbAYLIG{kbaeNIwwibQuKHwS]`qQCFw[a`AEDGGWd`aM]HiClgambIX{fhQqcIi?glb}{PJ\\@nSUwOjk}mSAyOxksdq}\\JhcoeBUYKXWrfBJMTk|VsDbPU\\HYtTvSVLLZpc^IR[lKqCe@Vd}AWEE@WeUAWuQBXeeBXub";

    pub const SNARK_IN_S6_10_PETERSEN: &str = ":Ig?SPc_EOrOFCQN";

    pub const SNARK_IN_BA_10_PETERSEN: &str =
        "\n1\n10\n4 6 8 \n5 6 9 \n4 7 9 \n5 7 8 \n0 2 5 \n1 3 4 \n0 1 7 \n2 3 6 \n0 3 9 \n1 2 8 \n";

    pub const SNARK_IN_ADJ_10_PETERSEN: &str = "0000101010\n0000011001\n0000100101\n0000010110\n1010010000\n0101100000\n1100000100\n0011001000\n1001000001\n0110000010\n";

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

    pub const SNARK_IN_G6_76: &str =
        "~?@KhDGHEG?G?_@?@?O_@G?c??C??G??G??C??@??@G?@@?O?P??AO????_???G???@???CC????G???@G????C????P????P??????_????@?????@??????_????CG?????H?????AO??????G??????G??????C??????@???????G??????C_?????AA?????O?P??????@G????????G???????@????????C???????GG????????G????????c????????@????????AG???????@C?????????@?????????@??????????_?????????G?????????`??????????c?????????C_??????????G??????????C??????????@???????????G???????????a??????????H??????????AA?????????G?Go??????????Q??G???C???A??";
}
