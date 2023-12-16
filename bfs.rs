
use std::collections::{HashMap, VecDeque};

#[cfg(test)]
mod tests {
    use super::bfs_with_path;

    #[test]
    fn test_bfs_with_path() {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
        graph.insert("B".to_string(), vec!["D".to_string()]);
        graph.insert("C".to_string(), vec!["E".to_string()]);
        graph.insert("D".to_string(), vec!["F".to_string()]);

        let mut expected_result: HashMap<String, (usize, Vec<String>)> = HashMap::new();
        expected_result.insert("A".to_string(), (0, vec!["A".to_string()]));
        expected_result.insert("B".to_string(), (1, vec!["A".to_string(), "B".to_string()]));
        expected_result.insert("C".to_string(), (1, vec!["A".to_string(), "C".to_string()]));
        expected_result.insert("D".to_string(), (2, vec!["A".to_string(), "B".to_string(), "D".to_string()]));
        expected_result.insert("E".to_string(), (2, vec!["A".to_string(), "C".to_string(), "E".to_string()]));
        expected_result.insert("F".to_string(), (3, vec!["A".to_string(), "B".to_string(), "D".to_string(), "F".to_string()]));

        let result = bfs_with_path(&graph, "A");

        assert_eq!(result, expected_result);
    }
}


pub fn bfs_with_path(graph: &HashMap<String, Vec<String>>, start_node: &str) -> HashMap<String, (usize, Vec<String>)> {
    let mut visited: HashMap<String, (usize, Vec<String>)> = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back(start_node.to_string());
    visited.insert(start_node.to_string(), (0, vec![start_node.to_string()]));

    while let Some(node) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                if !visited.contains_key(neighbor) {
                    let distance = visited[&node].0 + 1;
                    let mut path = visited[&node].1.clone();
                    path.push(neighbor.clone());
                    visited.insert(neighbor.clone(), (distance, path));
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    visited
}
