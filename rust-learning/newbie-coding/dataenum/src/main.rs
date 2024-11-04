enum Color {
    Rojo,
    Verde,
    Azul,
}

fn main() {
    let color1 = Color::Rojo;
    let color2 = Color::Verde;
    let color3 = Color::Azul;
    match color1 {
        Color::Rojo => println!("El color1 es rojo"),
        Color::Verde => println!("El color1 es verde"),
        Color::Azul => println!("El color1 es azul"),
    }
    match color2 {
        Color::Rojo => println!("El color2 es rojo"),
        Color::Verde => println!("El color2 es verde"),
        Color::Azul => println!("El color2 es azul"),
    }
    match color3 {
        Color::Rojo => println!("El color3 es rojo"),
        Color::Verde => println!("El color3 es verde"),
        Color::Azul => println!("El color3 es azul"),
    }
}
