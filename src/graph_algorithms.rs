use crate::graph as g;
use std::collections::VecDeque;
/// find shortest path using dijkstra method
///
/// # arguments
///
/// * start - starting node
///
/// * end - ending node
///
/// * dys_func - function which take versicle and return `usize` distance
///
/// # returns
///
/// it returns tuple of distance and vec of nodes (shortest path) or `None`    
pub fn dijkstra<V, N>(
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
                let new_dys = dys_func(graph.get_ver_value(current, neighbor).as_ref().unwrap())
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
        current = node;
    }
    result.reverse();
    Some((distance[end], result))
}

/// finds all nodes in given graph
///
/// # arguments
///
/// * graph - graph in witch you are looking for cycles
///
/// # return
///
/// * if any cycles are found vec of vec of usize that represents nodes if not None
pub fn find_all_cycles<V, N>(graph: &g::Graph<V, N>) -> Option<Vec<Vec<usize>>> {
    if graph.is_empty() {
        return None;
    }

    let mut result: Vec<Vec<usize>> = Vec::new();
    let mut visited = vec![false; graph.get_nodes_amount()];

    let mut stack = VecDeque::new();
    while !visited.iter().fold(true, |a, b| a & b) {
        println!("visited : {:?}", visited);
        let next = visited.iter().position(|&r| !r);
        match next {
            Some(n) => {
                visited[n] = true;
                println!("n = {}", n);
                if let Some(neighbors) = graph.get_neighbors(n) {
                    for neighbor in neighbors {
                        stack.push_back(vec![n, neighbor]);
                    }
                }
            }
            None => break,
        }
        while let Some(current) = stack.pop_back() {
            visited[*current.last()?] = true;
            if let Some(neighbors) = graph.get_neighbors(*current.last()?) {
                for neighbor in neighbors {
                    if let Some(pos) = current.iter().position(|&r| r == neighbor) {
                        result.push(current[pos..].to_vec());
                        continue;
                    }
                    let mut new_one = current.clone();
                    new_one.push(neighbor);
                    stack.push_back(new_one);
                }
            }
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}
