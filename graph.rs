use std::collections::HashMap;
use std::error::Error;
use plotters::prelude::*;
use plotters::style::RGBColor;
use std::io::{BufRead, BufReader};
use std::fs::{File};


pub struct NodeDegree {
    pub node: String,
    pub degree: f64,
}

fn get_degree_color(degree: usize) -> RGBColor {
    match degree {
        1 => RGBColor(255, 0, 0),    
        2 => RGBColor(0, 0, 139), 
        3 => RGBColor(25, 95, 40),  
        4 => RGBColor(128, 0, 128), 
        _ => RGBColor(0, 0, 0),     
    }
}

// Function to create and display the graph
pub fn display_graph(
    degree_limit: f64,
    starting_node_string: &String, 
    data: &[NodeDegree],
    degrees: &HashMap<String, (usize, Vec<String>)>,
) -> Result<(), Box<dyn Error>> {    // Read the data from the text file
    let file = File::open("output.txt")?;
    let reader = BufReader::new(file);

    let mut data: Vec<NodeDegree> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let node: f64 = parts[0].parse()?;
            let degree: f64 = parts[1].parse()?;
            data.push(NodeDegree { node, degree });
        }
    }

    // Set up the plot
    let filename = format!("degree_plot_{}.png", degree_limit);
    let root = BitMapBackend::new(&filename, (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Connectivity for Degree of Separation: {}", degree_limit), ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..1901f64, 0f64..5f64)?;

    chart
        .configure_mesh()
        .x_desc("Node number")
        .y_desc("Degrees of Separation from Starting Node")
        .draw()?;


    // Plot the nodes and their degrees of separation
    chart
        .draw_series(data.iter().map(|point| {
            Circle::new(
                (point.node, point.degree),
                1,
                ShapeStyle::from(&BLACK).filled(),
            )
        }))?;

    // Highlight connectivity based on degrees of separation
    let starting_node_degree = NodeDegree { node: 1.0, degree: 0.0 };

    for node in data {
        // Modify this part to handle string comparison
        let degree_of_separation = if node.node == starting_node_string {
            0
        } else {
            // Calculate degree of separation for other nodes
            (node.degree - starting_node_degree.degree).abs() as usize
        };
    
        if degree_of_separation == degree_limit as usize {
            let color = get_degree_color(degree_of_separation);
            let line_style = ShapeStyle::from(&color).stroke_width(1);
    
            let path = &degrees[&format!("{}", node.node)].1; 
    
            // Draw connections between nodes in the path
            for i in 0..path.len() - 1 {
                let source_node = path[i].parse::<f64>().unwrap();
                let target_node = path[i + 1].parse::<f64>().unwrap();
                chart.draw_series(std::iter::once(PathElement::new(
                    vec![(source_node, i as f64), (target_node, (i + 1) as f64)],
                    line_style,
                )))?;
            }
        }
    }
    Ok(())
}