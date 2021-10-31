mod graph_tests{
    use crate::graph::Graph;

    #[test]
    fn adding_nodes_works() {
        let mut g: Graph<i32, i32> = Graph::new();
        assert_eq!(g.get_nodes_amount(), 0);

        assert_eq!(g.add_node(1), 0);
        assert_eq!(g.add_node(2), 1);
        assert_eq!(g.add_node(3), 2);
        assert_eq!(g.add_node(4), 3);

        assert_eq!(g.get_nodes_amount(), 4);

        assert_eq!(g.get_node_value(0), Some(&1));
        assert_eq!(g.get_node_value(1), Some(&2));
        assert_eq!(g.get_node_value(2), Some(&3));
        assert_eq!(g.get_node_value(3), Some(&4));
        assert_eq!(g.get_node_value(4), None);
    }
    #[test]
    fn adding_vertices_works() {
        let mut g: Graph<i32, i32> = Graph::new();
        for i in 0..10 {
            g.add_node(i);
        }

        assert_eq!(g.add_ver(0, 1, 1), Ok(()));
        assert_eq!(g.add_ver(1, 2, 2), Ok(()));
        assert_eq!(g.add_ver(2, 3, 4), Ok(()));
        assert_eq!(g.add_ver(9, 1, 9), Ok(()));
        assert_eq!(g.add_ver(4, 5, 1), Ok(()));
        assert_eq!(g.add_ver(11, 2, 1), Err("node number out of range"));
    }
    #[test]
    fn removing_nodes_works() {
        let mut g: Graph<i32, i32> = Graph::new();
        for i in 0..10 {
            g.add_node(i);
        }
        assert_eq!(g.get_nodes_amount(), 10);
        assert_eq!(g.get_node_value(0),Some(&0));
        assert_eq!(g.delate_node(0), Ok(()));
        assert_eq!(g.get_node_value(0),Some(&1));

        assert_eq!(g.get_nodes_amount(), 9);
    }
    #[test]
    fn removing_ver_works() {
        let mut g: Graph<i32, i32> = Graph::new();
        for i in 0..10 {
            g.add_node(i);
        }
        for i in 0..10 {
            for p in 0..10 {
                assert_eq!(g.add_ver(i, p, (i * p) as i32), Ok(()));
            }
        }
        assert_eq!(g.delate_versicles(0, 1), Ok(()));
        assert_eq!(g.delate_versicles(0, 1), Err("trying to delate non-existing verticle"));
        assert_eq!(g.get_ver_value(0, 1),&None);
        assert_eq!(g.delate_versicles(11, 11),Err("node number out of range"));
    }

    // #[test]
    // fn generate_dot_file(){
    //     let mut g: Graph<i32, i32> = Graph::new();
    //     for i in 0..5 {
    //         g.add_node(i);
    //     }
    //     g.add_ver(0, 1, 1);
    //     g.add_ver(0, 2, 1);
    //     g.add_ver(0, 3, 1);
    //     g.add_ver(0, 4, 1);
    //     g.add_ver(1, 2, 1);
    //     g.add_ver(3, 1, 1);
    //     g.add_ver(4, 3, 1);

    //     g.to_dot("file_name.txt");
    // }
}
