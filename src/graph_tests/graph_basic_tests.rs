use crate::{grap, graph::Graph, graph::GraphError};

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
    assert_eq!(g.add_ver(11, 2, 1), Err(GraphError::NodeOutOfRange));
}
#[test]
fn removing_nodes_works() {
    let mut g: Graph<i32, i32> = Graph::new();
    for i in 0..10 {
        g.add_node(i);
    }
    assert_eq!(g.get_nodes_amount(), 10);
    assert_eq!(g.get_node_value(0), Some(&0));
    assert_eq!(g.delete_node(0), Ok(()));
    assert_eq!(g.get_node_value(0), Some(&1));

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
    assert_eq!(g.delete_versicles(0, 1), Ok(()));
    assert_eq!(
        g.delete_versicles(0, 1),
        Err(GraphError::RemovingNonExistantNode)
    );
    assert_eq!(g.get_ver_value(0, 1), &None);
    assert_eq!(g.delete_versicles(11, 11), Err(GraphError::NodeOutOfRange));
}

#[test]
fn macro_test() {
    let g: Graph<i32, i32> = grap!(0,1,2,3,4;(0,1,0),(1,2,3));
    for i in 0..5 {
        assert_eq!(g.get_node_value(i), Some(&(i as i32)));
    }
    assert_eq!(g.get_ver_value(0, 1), &Some(0));
    assert_eq!(g.get_ver_value(1, 2), &Some(3));
}
