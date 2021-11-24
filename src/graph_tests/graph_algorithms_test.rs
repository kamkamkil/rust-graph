mod dijkstra_test {
    use crate::graph::Graph;
    use crate::graph_algorithms::dijkstra;
    #[test]
    fn straight_path() {
        let mut g: Graph<usize, usize> = Graph::new();
        g.add_node(1);
        g.add_node(2);
        g.add_node(3);
        g.add_node(4);
        assert_eq!(g.add_ver(0, 1, 1), Ok(()));
        assert_eq!(g.add_ver(1, 2, 1), Ok(()));
        assert_eq!(g.add_ver(2, 3, 1), Ok(()));

        assert_eq!(dijkstra(&g, 0, 3, |x| *x), Some((3, [0, 1, 2, 3].to_vec())));
    }
    #[test]
    fn no_path() {
        let mut g: Graph<usize, usize> = Graph::new();
        g.add_node(1);
        g.add_node(2);
        g.add_node(3);
        g.add_node(4);
        assert_eq!(g.add_ver(0, 1, 1), Ok(()));
        assert_eq!(g.add_ver(0, 2, 1), Ok(()));

        assert_eq!(dijkstra(&g, 0, 3, |x| *x), None);
    }

    #[test]
    fn complex_graph() {
        let mut g: Graph<usize, usize> = Graph::new();
        g.add_node(0);
        g.add_node(1);
        g.add_node(2);
        g.add_node(3);
        g.add_node(4);
        g.add_node(5);
        g.add_node(6);
        g.add_node(7);
        g.add_node(8);

        assert_eq!(g.add_ver(0, 1, 1), Ok(()));
        assert_eq!(g.add_ver(0, 2, 2), Ok(()));
        assert_eq!(g.add_ver(2, 0, 2), Ok(()));
        assert_eq!(g.add_ver(1, 2, 1), Ok(()));
        assert_eq!(g.add_ver(3, 4, 1), Ok(()));
        assert_eq!(g.add_ver(1, 5, 1), Ok(()));
        assert_eq!(g.add_ver(5, 6, 1), Ok(()));
        assert_eq!(g.add_ver(0, 5, 10), Ok(()));
        assert_eq!(g.add_ver(6, 7, 1), Ok(()));
        assert_eq!(g.add_ver(7, 8, 1), Ok(()));
        assert_eq!(g.add_ver(2, 3, 1), Ok(()));
        assert_eq!(g.add_ver(4, 8, 5), Ok(()));

        assert_eq!(
            dijkstra(&g, 0, 8, |x| *x),
            Some((5, [0, 1, 5, 6, 7, 8].to_vec()))
        );
    }
}
mod find_all_cycles_test {
    use crate::grap;
    use crate::graph::Graph;
    use crate::graph_algorithms::find_all_cycles;

    fn vec_mach_after_rotation<T: std::cmp::PartialEq>(
        result: &Vec<T>,
        correct: &mut Vec<T>,
    ) -> Result<(), String> {
        if result.len() != correct.len() {
            return Err("String lengh doesn't match".to_string());
        }
        for _ in 0..result.len() {
            if correct == result {
                return Ok(());
            } else {
                correct.rotate_right(1);
            }
        }
        Err("vec Doesn't match".to_string())
    }
    #[test]
    fn none_cycles() {
        let g = grap!(0,1,2,3,4;(0,1,0),(1,2,0),(2,3,1),(3,4,3));
        assert_eq!(find_all_cycles(&g), None);
    }

    #[test]
    fn single_cycle() {
        let g = grap!(0,1,2,3,4;(0,1,0),(1,2,0),(2,3,0),(3,4,0),(4,0,0));
        let res = find_all_cycles(&g);
        match res {
            Some(r) => assert_eq!(r[0], [0, 1, 2, 3, 4].to_vec()),
            None => assert!(false, "no path was found but there should be one \n"),
        }
    }

    #[test]
    fn two_cycles() {
        let g = grap!(0,1,2,3,4;(0,1,0),(1,2,0),(2,3,0),(3,4,0),(4,0,0),(2,0,0));
        let res = find_all_cycles(&g);
        match res {
            Some(r) => assert_eq!(r, [[0, 1, 2].to_vec(), [0, 1, 2, 3, 4].to_vec()].to_vec()),
            None => assert!(false, "no path was found but there should be one \n"),
        }
    }
    #[test]
    fn multiple_cycles() {
        let g = grap!(0,1,2,3,4,5,6;
                                   (0,1,0),(1,2,0),(2,3,0),(3,4,0),(4,5,0),(5,0,0),(6,2,0),(6,0,0),(6,1,0),(4,6,0),(5,6,0),(3,6,0));
        let res = find_all_cycles(&g).unwrap();
        // g.to_dot("file_name.dot");
        let mut corr: Vec<Vec<usize>> = [
            [0, 1, 2, 3, 6].to_vec(),
            [1, 2, 3, 6].to_vec(),
            [2, 3, 6].to_vec(),
            [0, 1, 2, 3, 4, 6].to_vec(),
            [1, 2, 3, 4, 6].to_vec(),
            [2, 3, 4, 6].to_vec(),
            [0, 1, 2, 3, 4, 5].to_vec(),
            [0, 1, 2, 3, 4, 5, 6].to_vec(),
            [1, 2, 3, 4, 5, 6].to_vec(),
            [2, 3, 4, 5, 6].to_vec(),
        ]
        .to_vec();
        for r in &res {
            let mut test = false;
            for c in &mut corr {
                if let Ok(_) = vec_mach_after_rotation(&r, c) {
                    test = true;
                }
                if test {
                    break;
                }
            }
            assert!(test, "cykle {:?} should not be found", r);
        }
        assert!(res.len() == corr.len());
    }
    #[test]
    fn backtrace_cycle() {
        let g = grap!(0,1,2,3,4;(1,0,0),(1,2,0),(2,3,0),(3,1,0),(0,4,0));
        let res = find_all_cycles(&g);
        let mut corr: Vec<usize> = [1,2,3].to_vec();
        match res {
            Some(r) => assert_eq!(vec_mach_after_rotation(&r[0],&mut corr),Ok(())),
            None => assert!(false, "no path was found but there should be one \n"),
        }
    }
}
