use std::io;

fn main() {
    println!("Calculadora simple en Rust");

    println!("Ingrese el primer número:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Error al leer el número");
    let num1: f64 = num1.trim().parse().expect("Por favor, ingrese un número válido");

    // Aquí es donde ingresa el segundo número
    println!("Ingrese el segundo número:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Error al leer el número");
    let num2: f64 = num2.trim().parse().expect("Por favor, ingrese un número válido");

    // Aquí es donde se realiza la operación
    println!("Seleccione la operación: +, -, *, /");
    let mut operacion = String::new();
    io::stdin().read_line(&mut operacion).expect("Error al leer la operación");
    let operacion = operacion.trim();

    let result = match operacion {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("❌ Error: No se puede dividir entre cero");
                return;
            }
            num1 / num2
        },
        _ => {
            println!("❌ Operación no válida");
            return;
        }
    };

    println!("El resultado de {} {} {} es: {}", num1, operacion, num2, result);
    println!("Gracias por usar la calculadora simple en Rust!");
    println!("¡Hasta luego!");
}
