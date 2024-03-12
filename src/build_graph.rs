use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::dot::{Dot, Config};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::collections::HashMap;

pub fn build_graph_from_file(path: &Path) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut graph = DiGraph::<String, ()>::new();
    let mut node_indices = HashMap::<String, NodeIndex>::new();

    let mut current_file = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("File:") {
            current_file = line.replace("File:", "").trim().to_string();
        } else if line.contains("import") && line.contains("from") {
            let parts: Vec<&str> = line.split("from").collect();
            if let Some(import_part) = parts.get(1) {
                let imported_file = import_part.trim_matches(&['\'', ';'][..]).trim().to_string();

                let importer_index = *node_indices.entry(current_file.clone())
                    .or_insert_with(|| graph.add_node(current_file.clone()));
                
                let imported_index = *node_indices.entry(imported_file.clone())
                    .or_insert_with(|| graph.add_node(imported_file.clone()));

                graph.add_edge(importer_index, imported_index, ());
            }
        }
    }

    // Abre un nuevo archivo para escribir o sobrescribe el existente
    let mut file = File::create("graph.dot")?;
    
    // Escribe la representación DOT del grafo en el archivo
    write!(file, "{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]))?;

    Ok(())
}
