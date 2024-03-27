/*
    Ejercicio 1.

    Pedirle al usuario su nombre y su edad, y mostrar un mensaje que diga:
    "Hola [nombre], tienes [edad] años"
*/

use std::io::stdin;

pub fn hello_world() {
    print!("Escribe tu nombre: ");

    let console = stdin();
    let mut input = String::new();

    console
        .read_line(&mut input)
        .expect("This should not happen. Hope so!");

    input.retain(|c| c.is_alphabetic());
    let name = input.clone();

    let mut age: Option<u8> = None;

    while age.is_none() {
        print!("Escribe tu edad: ");

        console
            .read_line(&mut input)
            .expect("This should not happen. Hope so!");

        input.retain(|c| c.is_numeric());
        let result = input.parse::<u8>();

        match result {
            Ok(v) => age = Some(v),
            Err(_) => println!("El valor ingresado no es un número"),
        }
    }

    greeting(name, age.unwrap());
}

fn greeting(name: String, age: u8) {
    println!("Hola {name}, tienes {age} años.");
}
