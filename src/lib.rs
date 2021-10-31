#![allow(dead_code)]
pub mod graph {

    pub(crate) struct Graph<V, N> {
        versicles: Vec<Vec<Option<V>>>,
        nodes: Vec<N>,
        versicles_amount: usize,
    }

    impl<V: Clone, N> Graph<V, N> {
        pub fn add_node(&mut self, data: N) -> usize{
            self.nodes.push(data);
            self.versicles.push(vec![None;self.nodes.len() - 1 ]);
            for v in &mut self.versicles{
                v.push(None);
            }
            self.nodes.len() - 1
        }
        pub fn add_ver(&mut self, n1: usize, n2: usize, data: V) -> Result<(),&str>
        {
            if n1 < self.nodes.len() && n2 < self.nodes.len() {
                self.versicles_amount += 1;
                self.versicles[n1][n2] = Some(data);
                Ok(())
            }else{
                Err("node number out of range")
            }
        }

        pub fn delate_node(&mut self, node: usize) -> Result<(),&str> {
            if node >self.nodes.len(){
                Err("node number out of range")
            }else {
                self.nodes.remove(node);
                self.versicles.remove(node);
                for v in &mut self.versicles{
                    v.remove(node);
                }
                Ok(())
            }
        }

        pub fn delate_versicles(&mut self, n1: usize, n2: usize) -> Result<(),&str>
         {
            if n1 < self.nodes.len() && n2 < self.nodes.len(){
                self.versicles[n1][n2] = None;
                Ok(())
            }else{
                Err("node number out of range")
            }

        }

        pub fn get_nodes_amount(&self) -> usize {
            self.nodes.len()
        }

        pub fn get_versicles_amount(&self) -> usize {
            self.versicles_amount
        }

        pub fn get_ver_value(&mut self, n1: usize, n2: usize) -> &Option<V>
        {
            if n1 < self.nodes.len() && n2 < self.nodes.len() {
                &self.versicles[n1][n2]
            }else{
                &None
            }
        }
        
        pub fn get_node_value(&self,node: usize) -> Option<&N>
        {
            if node < self.nodes.len() {
                return Some(&self.nodes[node])
            }else{
                None
            }
        }

        pub fn new() -> Self{
            Graph { versicles: Vec::new(), nodes: Vec::new(), versicles_amount: 0 }
        }
    }


}
#[cfg(test)]
mod graph_test {
    use crate::graph::Graph ;

    #[test]
    fn adding_nodes_works() {
        let mut g: Graph<i32,i32> = Graph::new();
        assert_eq!(g.get_nodes_amount(), 0);

        assert_eq!(g.add_node(1),0);
        assert_eq!(g.add_node(2),1);
        assert_eq!(g.add_node(3),2);
        assert_eq!(g.add_node(4),3);

        assert_eq!(g.get_nodes_amount(), 4);

        assert_eq!(g.get_node_value(0),Some(&1));
        assert_eq!(g.get_node_value(1),Some(&2));
        assert_eq!(g.get_node_value(2),Some(&3));
        assert_eq!(g.get_node_value(3),Some(&4));
        assert_eq!(g.get_node_value(4),None);
    }
    #[test]
    fn adding_vertices_works(){ 
        let mut g: Graph<i32,i32> = Graph::new();
        for i in 0..10 {
            g.add_node(i);
        }

        assert_eq!(g.add_ver(0, 1, 1),Ok(()));
        assert_eq!(g.add_ver(1, 2, 2),Ok(()));
        assert_eq!(g.add_ver(2, 3, 4),Ok(()));
        assert_eq!(g.add_ver(9, 1, 9),Ok(()));
        assert_eq!(g.add_ver(4, 5, 1),Ok(()));
        assert_eq!(g.add_ver(11, 2, 1),Err("node number out of range"));
    }
    #[test]
    fn removing_nodes_works(){
        let mut g: Graph<i32,i32> = Graph::new();
        for i in 0..10 {
            g.add_node(i);
        }
    }

}
