
    use std::collections::VecDeque;

    use crate::graph::Graph;
    struct BFSIter<'a, V, N> {
        graph: &'a Graph<V, N>,
        current: usize,
        queue: VecDeque<V>,
    }

    impl<'a, V, N> BFSIter<'a, V, N> {
        pub fn new(graph: &'a Graph<V, N>, start: usize) -> Self {
            Self {
                graph,
                current : start,
                queue : VecDeque::new(),
            }
        }
    }

    pub fn func(){}


