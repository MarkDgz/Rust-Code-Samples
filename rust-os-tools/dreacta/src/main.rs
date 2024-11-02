use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::collections::HashMap;
use std::io;
use std::ffi::OsStr;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Función que busca archivos .tsx y .ts en un directorio de manera iterativa,
/// excluyendo el directorio `node_modules` y mostrando solo los paths relativos.
fn find_tsx_ts_files(_base_dir: &Path) -> io::Result<HashMap<PathBuf, Vec<PathBuf>>> {
    let mut file_tree: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
    let mut dirs_to_visit = vec![_base_dir.to_path_buf()];

    while let Some(dir) = dirs_to_visit.pop() {
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();

            // Excluir el directorio `node_modules`
            if path.is_dir() && path.file_name() == Some(OsStr::new("node_modules")) {
                continue;
            }

            // Si es un archivo con extensión .tsx o .ts, lo añadimos al directorio correspondiente
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "tsx" || ext == "ts" {
                        let relative_path = path.strip_prefix(_base_dir).unwrap_or(&path).to_path_buf();
                        let parent_dir = relative_path.parent().unwrap_or_else(|| Path::new("")).to_path_buf();
                        file_tree.entry(parent_dir).or_insert_with(Vec::new).push(relative_path);
                    }
                }
            }

            // Si es un directorio, lo añadimos a la lista de directorios a visitar
            if path.is_dir() {
                dirs_to_visit.push(path);
            }
        }
    }
    Ok(file_tree)
}

/// Función para imprimir la jerarquía de archivos con indentación para reflejar su estructura en el sistema de archivos.
fn print_file_tree(file_tree: &HashMap<PathBuf, Vec<PathBuf>>, _base_dir: &Path) {
    let mut directories: Vec<_> = file_tree.keys().collect();
    directories.sort();

    for dir in directories {
        // Imprimir el nombre del directorio
        if !dir.as_os_str().is_empty() {
            println!("Directory: {}", dir.display());
        }

        // Obtener y ordenar los archivos dentro del directorio
        if let Some(files) = file_tree.get(dir) {
            let mut sorted_files = files.clone();
            sorted_files.sort();

            for file in sorted_files {
                println!("  File: {}", file.display());
            }
        }
    }
}

fn main() -> io::Result<()> {
    // Mostrar la versión del programa
    println!("dreact - Dependency Analyzer for React/TypeScript Projects (v{})", VERSION);

    // Obtener el directorio actual donde se está ejecutando el programa
    let current_dir = env::current_dir()?;

    // Buscar archivos .tsx y .ts excluyendo node_modules
    let file_tree = find_tsx_ts_files(&current_dir)?;

    // Imprimir los archivos jerárquicamente
    print_file_tree(&file_tree, &current_dir);

    Ok(())
}
