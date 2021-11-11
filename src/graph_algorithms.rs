use crate::graph as g;

pub(crate) fn dijkstra<V: Clone, N>(
    graph: &g::Graph<V, N>,
    start: usize,
    end: usize,
    dys_func: impl Fn(&V) -> usize,
) -> Option<(usize, Vec<usize>)> {
    let mut distance = vec![usize::MAX; graph.get_nodes_amount()];
    distance[start] = 0;
    let mut prev = vec![None; graph.get_nodes_amount()];
    let mut unvisited = vec![true; graph.get_nodes_amount()];
    let mut current = start;
    loop {
        if current == end {
            break;
        }
        unvisited[current] = false;
        if let Some(neighbors) = graph.get_neighbors(current) {
            for neighbor in neighbors {
                let new_dys = dys_func(&graph.get_ver_value(current, neighbor).as_ref().unwrap())
                    + distance[current];
                if new_dys < distance[neighbor] {
                    distance[neighbor] = new_dys;
                    prev[neighbor] = Some(current);
                }
            }
        }
        let mut smallest_node = usize::MAX;
        let mut new_one = current;
        for (i, dys) in distance.iter().enumerate() {
            if unvisited[i] && dys < &smallest_node {
                smallest_node = *dys;
                new_one = i;
            }
        }
        if new_one == current {
            return None;
        }
        current = new_one;
    }
    let mut result = Vec::new();

    let mut current = end;
    result.push(current);
    while current != start {
        let node = prev[current]?; 
                result.push(node);
                current = node
        
    }
    result.reverse();
    Some((distance[end], result))
}
