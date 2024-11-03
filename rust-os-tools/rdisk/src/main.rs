use serde::Serialize;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;
use walkdir::WalkDir;
use std::env;

#[derive(Serialize)]
struct FileInfo {
    path: String,
    size: u64,
}

// Función para contar archivos y directorios en una ruta dada
fn get_total_files_and_dirs<P: AsRef<Path>>(dir: P) -> (usize, usize) {
    let mut file_count = 0;
    let mut dir_count = 0;

    for entry in WalkDir::new(dir) {
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                file_count += 1;
            } else if entry.file_type().is_dir() {
                dir_count += 1;
            }
        }
    }

    (file_count, dir_count)
}

// Función para obtener los archivos más grandes
fn get_largest_files<P: AsRef<Path>>(dir: P, limit: usize, total_files: usize) -> Vec<FileInfo> {
    println!("Iniciando etapa 1: Escaneo y recolección de archivos...");

    let mut files = Vec::new();
    let mut processed_files = 0;

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        if let Ok(metadata) = entry.metadata() {
            files.push(FileInfo {
                path: entry.path().display().to_string(),
                size: metadata.len(),
            });
        }

        // Actualizar el contador de archivos procesados y mostrar el progreso cada 100 archivos
        processed_files += 1;
        if processed_files % 100 == 0 || processed_files == total_files {
            let progress = (processed_files as f64 / total_files as f64) * 100.0;
            print!("\rProcesando archivos: {:.2}% completado", progress);
            io::stdout().flush().unwrap();
        }
    }

    println!("\nEtapa 1 completada.");

    // Ordenar archivos por tamaño en orden descendente y limitar a los primeros 500
    files.sort_by(|a, b| b.size.cmp(&a.size));
    files.into_iter().take(limit).collect()
}

// Función para guardar los datos en un archivo JSON
fn save_to_json<P: AsRef<Path> + Clone>(data: &[FileInfo], file_path: P) -> std::io::Result<()> {
    println!("Iniciando etapa 2: Guardado de resultados en archivo JSON...");
    
    let mut file = File::create(file_path.clone())?;
    let json_data = serde_json::to_string_pretty(data)?;
    file.write_all(json_data.as_bytes())?;

    println!("Etapa 2 completada. Archivo JSON generado en '{}'", file_path.as_ref().display());
    Ok(())
}

fn main() {

    // Obtener los argumentos de línea de comandos
    let args: Vec<String> = env::args().collect();

    // Verificar si se proporcionó un argumento
    if args.len() < 2 {
        eprintln!("Uso: {} <ruta>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];

    // Validar la ruta
    if !Path::new(path).exists() {
        eprintln!("La ruta especificada no existe: {}", path);
        std::process::exit(1);
    }

//    let mut directory = "/home"; // Ruta del directorio a escanear, puedes cambiarla
    let directory = path; // Ruta del directorio a escanear, puedes cambiarla
    let output_file = "largest_files.json"; // Nombre del archivo de salida
    let file_limit = 500;

    // Advertencia y confirmación del usuario
    println!("Advertencia: El proceso tiene dos etapas y puede tomar varios minutos.");
    println!("Etapa 1: Escaneo de directorios y archivos.");
    println!("Etapa 2: Guardado de los archivos más grandes en un archivo JSON.\n");
    println!("¿Desea continuar? (s/n):");

    // Confirmación del usuario
    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation).expect("Error de entrada");

    if confirmation.trim().to_lowercase() != "s" {
        println!("Proceso cancelado por el usuario.");
        return;
    }

    // Inicio del proceso y medición del tiempo total
    let start_time = Instant::now();

    // Contar archivos y directorios antes de la confirmación
    println!("Etapa 1: Escaneo de directorios y archivos... en proceso, espere...");
    let (total_files, total_dirs) = get_total_files_and_dirs(&directory);
    println!(
        "Se encontraron aproximadamente {} archivos y {} directorios para procesar.",
        total_files, total_dirs
    );

    // Ejecutar etapa 1: Obtener archivos más grandes
    let largest_files = get_largest_files(directory, file_limit, total_files);

    // Ejecutar etapa 2: Guardar resultados en JSON
    println!("Etapa 2: Guardando archivos resultado en un archivo JSON. Espere...\n");
    if let Err(e) = save_to_json(&largest_files, output_file) {
        eprintln!("Error al guardar el archivo JSON: {}", e);
    }

    // Cifras finales
    let duration = start_time.elapsed();
    println!("\nProceso completado.");
    println!("Tiempo total: {:.2?}", duration);
    println!("Archivos procesados: {}", total_files);
    println!("Directorios procesados: {}", total_dirs);
}
