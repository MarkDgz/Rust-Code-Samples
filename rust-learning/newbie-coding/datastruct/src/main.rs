struct Libro {
    titulo: String,
    autor: String,
    paginas: u32,
}

fn imprimir_info(libro: &Libro) {
    println!("Título: {}, Autor: {}, Páginas: {}", libro.titulo, libro.autor, libro.paginas);
}

fn main() {
    let libro = Libro {
        titulo: String::from("Cien años de soledad"),
        autor: String::from("Gabriel García Márquez"),
        paginas: 417,
    };

    imprimir_info(&libro);
}
