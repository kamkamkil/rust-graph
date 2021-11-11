use std::collections::VecDeque;

use crate::graph::Graph;

pub trait GetNodeNumber {
    fn get_node_number(&self) -> Option<&usize>;
}

pub(crate) struct BFSIter<'a, V, N> {
    graph: &'a Graph<V, N>,
    queue: VecDeque<usize>,
    unvisited: Vec<bool>,
}

impl<'a, V, N> BFSIter<'a, V, N> {
    pub fn new(graph: &'a Graph<V, N>, start: usize) -> Self {
        let mut new = Self {
            graph,
            queue: VecDeque::new(),
            unvisited: vec![true; graph.get_nodes_amount()],
        };
        new.queue.push_back(start);
        new
    }
}
// temporary unsafe
impl<'a, V, N> Iterator for BFSIter<'a, V, N> {
    type Item = &'a N;

    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop_front() {
            None => None,
            Some(x) => {
                self.unvisited[x] = false;
                for n in self.graph.get_neighbors(x)? {
                    if self.unvisited[n] {
                        self.queue.push_back(n);
                    }
                }
                Some(self.graph.get_node_value(x)?)
            }
        }
    }
}

impl<'a, V: Clone, N> GetNodeNumber for BFSIter<'a, V, N>
{
    fn get_node_number(&self) -> Option<&usize> {
        self.queue.back()
    }
}
pub(crate) struct DFSIter<'a, V, N> {
    graph: &'a Graph<V, N>,
    stack: VecDeque<usize>,
    unvisited: Vec<bool>,
}

impl<'a, V, N> DFSIter<'a, V, N> {
    pub fn new(graph: &'a Graph<V, N>, start: usize) -> Self {
        let mut new_one = Self {
            graph,
            stack : VecDeque::new(),
            unvisited: vec![true; graph.get_nodes_amount()],
        };
        new_one.stack.push_back(start);
        new_one
    }
}

impl<'a, V, N> Iterator for DFSIter<'a, V, N> {
    type Item = &'a N;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop_front() {
            None => None,
            Some(x) => {
                self.unvisited[x] = false;
                for n in self.graph.get_neighbors(x)? {
                    if self.unvisited[n] {
                        self.stack.push_front(n);
                    }
                }
                Some(self.graph.get_node_value(x)?)
            }
        }
    }
}

impl<'a, V: Clone, N> GetNodeNumber for DFSIter<'a, V, N>
{
    fn get_node_number(&self) -> Option<&usize> {
        self.stack.front()
    }
}