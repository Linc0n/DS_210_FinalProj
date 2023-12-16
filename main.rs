use std::collections::{HashMap};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::cmp::Reverse;
mod bfs;
mod graph;
use graph::NodeDegree;


fn main() {
    // Read the file
    if let Ok(file) = File::open("CollegeMsg.txt") {
        let reader = BufReader::new(file);
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();

        for line in reader.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() == 3 {
                    let sender = parts[0].to_string();
                    let receiver = parts[1].to_string();

                    // Add sender and receiver to the graph
                    graph.entry(sender.clone()).or_insert_with(Vec::new).push(receiver.clone());
                    graph.entry(receiver.clone()).or_insert_with(Vec::new).push(sender.clone());
                }
            }
        }

  // Start BFS from the node 1
  let starting_node_string = "1"; 
  let starting_node_degree = NodeDegree { node: 1.0, degree: 0.0 }; 
  if let Some(_) = graph.get(starting_node_string) {
    let degrees = bfs::bfs_with_path(&graph, starting_node_string); // Use bfs::bfs_with_path here


      // sorting by degree in descending order
      let mut sorted_degrees: Vec<_> = degrees.iter().filter(|&(node, _)| node != starting_node_string).collect();
      sorted_degrees.sort_by_key(|&(_, ref v)| Reverse(v.0));

      let lines_to_print = 100000; // Adjustable output mainly used to confirm correct output during earlier stages of the project
      let mut printed_lines = 0;
      let mut output_buffer = String::new();

      for (node, degree) in sorted_degrees {
        if degree.0 > 20000 {
            continue;
          } else {
              if printed_lines >= lines_to_print {
                  break; 
              }
              output_buffer.push_str(&format!("{} {}\n", node, degree.0));
              for string_value in &degree.1 {
                output_buffer.push_str(&format!("{}\n", string_value));
            }
              printed_lines += 1;
          }
      }

      // Write output to a file
      let output_file_name = "output.txt";
      let mut output_file = OpenOptions::new()
          .write(true)
          .truncate(true)
          .create(true)
          .open(output_file_name)
          .expect("Unable to create output file");

      output_file
          .write_all(output_buffer.as_bytes())
          .expect("Error writing to file");

      println!("Output written to {}", output_file_name);

      // Prepare data to be used by display_graph
      let file = File::open("output.txt").expect("Error opening file");
      let reader = BufReader::new(file);
      let mut data: Vec<NodeDegree> = Vec::new();

      for line in reader.lines() {
          let line = line.expect("Error reading line");
          let parts: Vec<&str> = line.split_whitespace().collect();
          if parts.len() >= 2 {
              let node: f64 = parts[0].parse().expect("Error parsing node");
              let degree: f64 = parts[1].parse().expect("Error parsing degree");
              data.push(NodeDegree { node, degree });
          }
      }
      // Create the five graphs
      for degree in 1..=5 {
        let starting_node_string = "1"; 
        let starting_node_degree = NodeDegree { node: 1.0, degree: 0.0 }; 
        if let Err(err) = graph::display_graph(degree as f64, &starting_node_degree, &data, &degrees) {
            eprintln!("Error displaying graph for degree {}: {}", degree, err);
        }
    }
    }
 } else {
    println!("The starting node is not found in the graph.");
}
}
    

