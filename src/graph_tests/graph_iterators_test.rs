#[cfg(test)]
mod bfs_iter_test {
    use crate::graph::to_dot;
    use crate::graph::Graph;
    #[test]
    fn basic_test() {
        let mut g: Graph<i32, i32> = Graph::new();
        for i in 0..6 {
            g.add_node(i);
        }
        assert_eq!(g.add_ver(0, 1, 1), Ok(()));
        assert_eq!(g.add_ver(0, 2, 2), Ok(()));
        assert_eq!(g.add_ver(0, 3, 3), Ok(()));
        assert_eq!(g.add_ver(2, 4, 4), Ok(()));
        assert_eq!(g.add_ver(4, 5, 5), Ok(()));
        assert_eq!(g.get_ver_value(4, 5), &Some(5));
        let vec: Vec<&i32> = g.bfs_iter(0).collect::<_>();
        assert_eq!(vec, [&0, &1, &2, &3, &4, &5]);
    }
    #[allow(unused_must_use)]
    #[test]
    fn test_with_loops() {
        let mut g: Graph<i32, i32> = Graph::new();
        for i in 0..6 {
            g.add_node(i);
        }
        assert_eq!(g.add_ver(0, 1, 1), Ok(()));
        assert_eq!(g.add_ver(0, 2, 2), Ok(()));
        assert_eq!(g.add_ver(0, 3, 3), Ok(()));
        assert_eq!(g.add_ver(2, 4, 4), Ok(()));
        assert_eq!(g.add_ver(4, 5, 5), Ok(()));
        assert_eq!(g.add_ver(4, 0, 5), Ok(()));
        assert_eq!(g.get_ver_value(4, 5), &Some(5));
        let vec: Vec<&i32> = g.bfs_iter(0).collect::<_>();
        assert_eq!(vec, [&0, &1, &2, &3, &4, &5]);
        g.to_dot_with_rules(
            "file_name.dot",
            to_dot::color_path_node(
                [1, 2].to_vec(),
                to_dot::NodeRule {
                    color: to_dot::Color::Black,
                    fill_color: to_dot::Color::Red,
                    label: true,
                    shape: to_dot::NodeShape::Ellipse,
                },
            ),
            to_dot::color_path_versicle(
                [(0,1),(0,2)].to_vec(),
                to_dot::VersicleRule{
                    color: to_dot::Color::Blue,
                    label: true,
                    style: to_dot::VersicleStyle::Curve,
                }
            )
        );
    }
}

#[cfg(test)]
mod dfs_iter_test {
    use crate::graph::Graph;

    #[test]
    fn basic_test() {
        let mut g: Graph<i32, i32> = Graph::new();
        for i in 0..6 {
            g.add_node(i);
        }
        assert_eq!(g.add_ver(0, 1, 1), Ok(()));
        assert_eq!(g.add_ver(0, 2, 2), Ok(()));
        assert_eq!(g.add_ver(0, 3, 3), Ok(()));
        assert_eq!(g.add_ver(2, 4, 4), Ok(()));
        assert_eq!(g.add_ver(4, 5, 5), Ok(()));
        assert_eq!(g.get_ver_value(4, 5), &Some(5));
        let vec: Vec<&i32> = g.dfs_iter(0).collect::<_>();
        assert_eq!(vec, [&0, &3, &2, &4, &5, &1]);
    }
    #[test]
    fn loop_test() {
        let mut g: Graph<i32, i32> = Graph::new();
        for i in 0..6 {
            g.add_node(i);
        }
        assert_eq!(g.add_ver(0, 1, 1), Ok(()));
        assert_eq!(g.add_ver(0, 2, 2), Ok(()));
        assert_eq!(g.add_ver(0, 3, 3), Ok(()));
        assert_eq!(g.add_ver(2, 4, 4), Ok(()));
        assert_eq!(g.add_ver(4, 5, 5), Ok(()));
        assert_eq!(g.add_ver(4, 0, 5), Ok(()));
        assert_eq!(g.get_ver_value(4, 5), &Some(5));
        let vec: Vec<&i32> = g.dfs_iter(0).collect::<_>();
        assert_eq!(vec, [&0, &3, &2, &4, &5, &1]);
    }
}
