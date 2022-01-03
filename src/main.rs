// Invocación de la libreria de inputs/outputs
use std::io;
// Invocación de la libreria de funciones random
use rand::Rng;


fn main() {
    println!("Adivina el número:");
    // CREACIÓN DE UN NÚMERO ALEATORIO
    let secret_num = rand::thread_rng().gen_range(1..999);

    // "let" creación de variable
    // "rand" llama a una funcion random y "thread_rng" nos da un generador de numeros random
    // "gen_range" nos da para generar un rango de numeros

    print!("You guessed: {}", secret_num);

    println!("Introduce un numero:");

    let mut guess=String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);



}
