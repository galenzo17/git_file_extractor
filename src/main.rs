use walkdir::{WalkDir, DirEntry};
use std::fs::File;
use std::io::{self, Write, Read};
use std::path::Path;
use std::env;
use dotenv::dotenv;
mod build_graph;
use std::process::Command;
use gif::{Encoder, Frame, Repeat};
use std::io::BufWriter;


fn is_target_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file() && 
    (entry.path().extension().map_or(false, |e| e == "ts" || e == "json"))
}
fn should_skip(entry: &DirEntry) -> bool {
    let skip_dirs = ["node_modules", "dist"];
    let file_name = entry.file_name().to_str().unwrap_or("");
    skip_dirs.contains(&file_name)
}
fn main() {
    dotenv().ok();
    let git_dir = env::var("GIT_REPO_PATH").expect("La variable de entorno GIT_REPO_PATH no está definida");
    
    let mut result_file = File::create("result.txt").expect("No se pudo crear el archivo result.txt");
    let mut token_counts_file = File::create("token_counts.txt").expect("No se pudo crear el archivo token_counts.txt");
    let mut total_tokens = 0;

    for entry in WalkDir::new(&git_dir)
        .into_iter()
        .filter_entry(|e| !should_skip(e))
        .filter_map(|e| e.ok())
        .filter(|e| is_target_file(e)) {
            let path = entry.path();
            let mut file = File::open(&path).expect("No se pudo abrir el archivo");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Error al leer el archivo");

            writeln!(result_file, "{}\n{}\n", path.display(), contents)
                .expect("Error al escribir en el archivo result.txt");
            let tokens = contents.split_whitespace().count();
            total_tokens += tokens;
    }
    writeln!(token_counts_file, "Total de tokens: {}", total_tokens)
        .expect("Error al escribir en el archivo token_counts.txt");
    build_graph::build_graph_from_file(Path::new("result.txt"));
}
