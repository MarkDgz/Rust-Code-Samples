use clap::Parser;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;
use walkdir::WalkDir;

#[derive(Parser)]
struct Args {
    /// Ruta inicial para el análisis
    path: String,

    /// Tipo de orden: "name", "size" o "count"
    #[arg(short, long, default_value = "name")]
    order_by: String,
}

#[derive(Serialize, Debug)]
struct DirectoryInfo {
    path: String,
    file_count: u64,
    total_size: u64,
    size_percentage: f64, // Porcentaje del tamaño total de archivos
}

fn main() {
    let args = Args::parse();

    let start_path = Path::new(&args.path);
    if !start_path.exists() || !start_path.is_dir() {
        eprintln!("La ruta proporcionada no existe o no es un directorio.");
        return;
    }

    // Advertencia y confirmación del usuario
    println!("Este proceso puede tardar varios minutos en completarse.");
    println!("¿Deseas continuar? (s/n): ");
    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation).expect("Error al leer la entrada");

    if confirmation.trim().to_lowercase() != "s" {
        println!("Proceso cancelado por el usuario.");
        return;
    }

    println!("Iniciando análisis en el directorio: {}", start_path.display());

    let start_time = Instant::now();
    let mut directories: HashMap<String, DirectoryInfo> = HashMap::new();
    let mut total_files = 0;
    let mut total_size = 0;

    // Contar archivos y directorios para el progreso
    let entries: Vec<_> = WalkDir::new(&args.path).into_iter().filter_map(|e| e.ok()).collect();
    let total_entries = entries.len();
    println!("Archivos y directorios encontrados: {}. Iniciando procesamiento...", total_entries);

    // Procesar archivos y mostrar progreso
    for (index, entry) in entries.into_iter().enumerate() {
        if entry.file_type().is_file() {
            let dir_path = entry.path().parent().unwrap().to_string_lossy().to_string();
            let file_size = entry.metadata().map(|m| m.len()).unwrap_or(0);

            directories
                .entry(dir_path.clone())
                .and_modify(|info| {
                    info.file_count += 1;
                    info.total_size += file_size;
                })
                .or_insert(DirectoryInfo {
                    path: dir_path,
                    file_count: 1,
                    total_size: file_size,
                    size_percentage: 0.0, // Se actualizará más adelante
                });

            total_files += 1;
            total_size += file_size;
        }

        // Actualizar progreso cada 100 archivos
        if index % 100 == 0 {
            println!("Progreso: {}/{} elementos procesados", index + 1, total_entries);
        }
    }

    // Calcular el porcentaje de tamaño para cada directorio
    for dir_info in directories.values_mut() {
        dir_info.size_percentage = (dir_info.total_size as f64 / total_size as f64) * 100.0;
    }

    // Convertir el HashMap a un Vec y ordenar según el criterio
    let mut dir_info_vec: Vec<DirectoryInfo> = directories.into_values().collect();
    match args.order_by.as_str() {
        "size" => dir_info_vec.sort_by(|a, b| b.total_size.cmp(&a.total_size)),
        "count" => dir_info_vec.sort_by(|a, b| b.file_count.cmp(&a.file_count)),
        _ => dir_info_vec.sort_by(|a, b| a.path.cmp(&b.path)),
    }

    println!("Procesamiento completado. Guardando en JSON...");

    // Guardar en un archivo JSON
    let output_file = "directory_info.json";
    let file = File::create(output_file).expect("No se pudo crear el archivo JSON");
    serde_json::to_writer_pretty(file, &dir_info_vec).expect("Error al escribir JSON");

    // Resumen final
    let duration = start_time.elapsed();
    println!("Análisis completado. Resultados guardados en {}", output_file);
    println!("Total de archivos procesados: {}", total_files);
    println!("Total de directorios procesados: {}", dir_info_vec.len());
    println!("Tamaño total de archivos: {} bytes", total_size);
    println!("Tiempo total de ejecución: {:?}", duration);
}

