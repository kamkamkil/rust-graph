#![allow(dead_code)]
pub mod graph {
    use std::iter;

    pub mod graph_iterators;




    pub(crate) struct Graph<V, N> {
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
            g.add_ver(ver1,ver2,data);
        )*
        g
    }}
    }

    impl<V, N> Graph<V, N> {
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
        pub fn add_ver(&mut self, n1: usize, n2: usize, data: V) -> Result<(), &str> {
            if n1 < self.nodes.len() && n2 < self.nodes.len() {
                self.versicles_amount += 1;
                self.versicles[n1][n2] = Some(data);
                Ok(())
            } else {
                Err("node number out of range")
            }
        }

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

        pub fn delate_versicles(&mut self, n1: usize, n2: usize) -> Result<(), &str> {
            if n1 < self.nodes.len() && n2 < self.nodes.len() {
                match self.versicles[n1][n2] {
                    None => Err("trying to delate non-existing verticle"),
                    _ => {
                        self.versicles[n1][n2] = None;
                        Ok(())
                    }
                }
            } else {
                Err("node number out of range")
            }
        } 

        pub fn get_nodes_amount(&self) -> usize {
            self.nodes.len()
        }

        pub fn get_versicles_amount(&self) -> usize {
            self.versicles_amount
        }
        //TODO dodać tutaj mutowalne wartości 
        pub fn get_ver_value(&self, n1: usize, n2: usize) -> &Option<V> {
            if n1 < self.nodes.len() && n2 < self.nodes.len() {
                &self.versicles[n1][n2]
            } else {
                &None
            }
        }

        pub fn get_node_value(&self, node: usize) -> Option<&N> {
            if node < self.nodes.len() {
                return Some(&self.nodes[node]);
            } else {
                None
            }
        }

        pub fn get_neighbors(&self, node: usize) -> Option<Vec<usize>>
        {
            if node > self.get_nodes_amount(){
                return None;
            }
            let mut result = Vec::new();
            
            for i  in 0 .. self.get_nodes_amount() {
                if let Some(_) = self.get_ver_value(node,i){
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
        pub fn bfs_iter(&self,node: usize) -> graph_iterators::BFSIter<V, N> {
            graph_iterators::BFSIter::new(&self, node).into_iter()
        }
        pub fn dfs_iter(&self,node: usize) -> graph_iterators::DFSIter<V, N> {
            graph_iterators::DFSIter::new(&self, node).into_iter()
        }
    }
    pub mod to_dot;
}

pub mod graph_algorithms; 

#[cfg(test)]
mod graph_tests ;