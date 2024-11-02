use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::io;

// Macro para obtener la versión del programa desde Cargo.toml
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Función que busca archivos .tsx y .ts en un directorio de manera recursiva,
/// excluyendo el directorio `node_modules` y mostrando solo los paths relativos.
fn find_tsx_ts_files(dir: &Path, base_dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            // Excluir el directorio `node_modules`
            if path.is_dir() && path.file_name().unwrap() == "node_modules" {
                continue;
            }

            // Si es un archivo con extensión .tsx o .ts, lo añadimos a la lista
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "tsx" || ext == "ts" {
                        // Convertir el path a uno relativo desde el directorio base
                        if let Ok(relative_path) = path.strip_prefix(base_dir) {
                            files.push(relative_path.to_path_buf());
                        }
                    }
                }
            }

            // Si es un directorio, hacemos la llamada recursiva
            if path.is_dir() {
                find_tsx_ts_files(&path, base_dir, files)?;
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    // Mostrar la versión del programa
    println!("dreact - Dependency Analyzer for React/TypeScript Projects (v{})", VERSION);
    
    // Obtiene el directorio actual donde se está ejecutando el programa
    let current_dir = env::current_dir()?;
    
    // Vector para almacenar los archivos encontrados
    let mut tsx_ts_files: Vec<PathBuf> = Vec::new();
    
    // Llama a la función recursiva para buscar los archivos, excluyendo node_modules
    find_tsx_ts_files(&current_dir, &current_dir, &mut tsx_ts_files)?;
    
    // Ordena la lista de archivos
    tsx_ts_files.sort();
    
    // Imprime los archivos encontrados como paths relativos
    for file in tsx_ts_files {
        println!("{}", file.display());
    }
    
    Ok(())
}
