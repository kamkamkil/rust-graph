#![allow(dead_code)]
pub mod graph {
    //! implementation of graph data struct based on association matrix 


    use std::iter;

    pub mod graph_iterators;
    pub struct Graph<V, N> {
        versicles: Vec<Vec<Option<V>>>,
        nodes: Vec<N>,
        versicles_amount: usize,
    }


    /// macro doesn't check if verisicles are correctly set  
    ///
    #[macro_export]
    macro_rules! grap{
        ($($node:expr),* ; $(($ver1:expr,$ver2:expr,$data:expr)),*)=>
        {{
            let mut g : Graph<_,_>= Graph::new();
            $(
                let node = $node;
                g.add_node(node);
            )*
            $(
                let ver1 = $ver1;
                let ver2 = $ver2;
                let data = $data;
                if let Ok(_) = g.add_ver(ver1,ver2,data) {}
        )*
        g
    }}
    }

    impl<V, N> Graph<V, N> {

        /// add node to graph 
        /// 
        /// # Arguments 
        /// 
        /// * `data` - data that node will hold 
        /// 
        /// # return 
        /// 
        /// * usize - number of your new node 
        /// 
        /// # Example
        /// 
        // / ```rust 
        // / let mut g : Graph<usize, usize> = Graph::new();
        // / assert_eq (g.add_node(1),0);
        // / ```
        pub fn add_node(&mut self, data: N) -> usize {
            self.nodes.push(data);
            self.versicles.push(
                iter::repeat_with(|| None)
                    .take(self.nodes.len() - 1)
                    .collect::<Vec<_>>(),
            );
            for v in &mut self.versicles {
                v.push(None);
            }
            self.nodes.len() - 1
        }

        /// adds vericles to graph 
        /// 
        /// # arguments 
        /// 
        /// * node1, node 2 - nodes between which new versicle will be created
        /// 
        /// * data - data which will be saved on new versicle 
        /// 
        /// # returns
        /// 
        /// * Ok(()) if everything is fine 
        /// 
        /// * Err("node number out of range") if any node is out of range 
        /// 
        pub fn add_ver(&mut self, node1: usize, node2: usize, data: V) -> Result<(), &str> {
            if node1 < self.nodes.len() && node2 < self.nodes.len() {
                self.versicles_amount += 1;
                self.versicles[node1][node2] = Some(data);
                Ok(())
            } else {
                Err("node number out of range")
            }
        }

        /// delate node and all versicles going in and out of it 
        /// 
        /// # arguments 
        /// 
        /// * node - node that will be deleted  
        /// 
        /// # return 
        /// 
        /// * OK(()) if everything is fine 
        /// 
        /// * Err("node number out of range") if any node is out of range 
        pub fn delate_node(&mut self, node: usize) -> Result<(), &str> {
            if node > self.nodes.len() {
                Err("node number out of range")
            } else {
                self.nodes.remove(node);
                self.versicles.remove(node);
                for v in &mut self.versicles {
                    v.remove(node);
                }
                Ok(())
            }
        }

        /// delate versicle 
        /// 
        /// # arguments
        /// 
        /// * node1,node2 - nodes between which  versicle will be deleted
        /// 
        /// # return 
        /// 
        /// * OK(()) if everything is fine 
        /// 
        /// * Err("node number out of range") if any node is out of range 
        /// 
        /// * Err("trying to delate non-existing verticle") if non existing versicles is being deleted 
        pub fn delate_versicles(&mut self, node1: usize, node2: usize) -> Result<(), &str> {
            if node1 < self.nodes.len() && node2 < self.nodes.len() {
                match self.versicles[node1][node2] {
                    None => Err("trying to delate non-existing verticle"),
                    _ => {
                        self.versicles[node1][node2] = None;
                        Ok(())
                    }
                }
            } else {
                Err("node number out of range")
            }
        }

        /// returns amount of nodes 
        pub fn get_nodes_amount(&self) -> usize {
            self.nodes.len()
        }

        /// returns amount of versicles 
        pub fn get_versicles_amount(&self) -> usize {
            self.versicles_amount
        }
        pub fn get_ver_value(&self, n1: usize, n2: usize) -> &Option<V> {
            if n1 < self.nodes.len() && n2 < self.nodes.len() {
                &self.versicles[n1][n2]
            } else {
                &None
            }
        }

        /// return value of passed code
        pub fn get_node_value(&self, node: usize) -> Option<&N> {
            if node < self.nodes.len() {
                return Some(&self.nodes[node]);
            } else {
                None
            }
        }

        /// return vector of nodes which are neighbors of given node 
        pub fn get_neighbors(&self, node: usize) -> Option<Vec<usize>> {
            if node > self.get_nodes_amount() {
                return None;
            }
            let mut result = Vec::new();

            for i in 0..self.get_nodes_amount() {
                if let Some(_) = self.get_ver_value(node, i) {
                    result.push(i);
                }
            }
            Some(result)
        }

        pub fn new() -> Self {
            Graph {
                versicles: Vec::new(),
                nodes: Vec::new(),
                versicles_amount: 0,
            }
        }

        /// returns bfs iterator starting from given node 
        pub fn bfs_iter(&self, node: usize) -> graph_iterators::BFSIter<V, N> {
            graph_iterators::BFSIter::new(&self, node).into_iter()
        }

        /// returns dfs iterator starting from given node
        pub fn dfs_iter(&self, node: usize) -> graph_iterators::DFSIter<V, N> {
            graph_iterators::DFSIter::new(&self, node).into_iter()
        }
    }
    pub mod to_dot;
}

pub mod graph_algorithms;

#[cfg(test)]
mod graph_tests;
