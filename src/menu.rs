/*
Opciones a crear
1.añadir_ciudad
2.añadir_conexion
3.Ruta mas corta con bfs
4.generar imagen */
// src/menu.rs

use petgraph::graph::UnGraph;
use std::io::{self, Write};
use crate::funciones; //para acceder a las funciones

pub fn iniciar_menu(mapa_ciudades: &mut UnGraph<String, u32>) {
    loop {
        println!("\n=================================");
        println!("   MENÚ DE GESTIÓN DE RUTAS");
        println!("=================================");
        println!("1. Añadir Ciudad");
        println!("2. Añadir Conexión");
        println!("3. Buscar Ruta Corta (A*)");
        println!("4. GENERAR IMAGEN DEL GRAFO ");
        println!("5. Salir");
        print!("Seleccione una opción (1-5): ");
        io::stdout().flush().unwrap();

        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).expect("Error al leer la opción");
        let opcion = opcion.trim();

        match opcion {
            "1" => {

            }
            "2" => {

            }
            "3" => {
                if mapa_ciudades.node_count() < 2 {
                    println!(" Se necesitan al menos 2 ciudades para buscar una ruta");
                    continue;
                }

                print!("Ingrese la ciudad de origen: ");
                io::stdout().flush().unwrap();
                let mut origen = String::new();
                io::stdin().read_line(&mut origen).expect("Error al leer la ciudad");
                let origen = origen.trim().to_string();

                print!("Ingrese la ciudad de destino: ");
                io::stdout().flush().unwrap();
                let mut destino = String::new();
                io::stdin().read_line(&mut destino).expect("Error al leer la ciudad");
                let destino = destino.trim().to_string();

                println!("\n🔍 Buscando la ruta más corta...");
                match funciones::ruta_mas_corta(mapa_ciudades, origen, destino) {
                    Some((distancia, ruta)) => {
                        println!(" Ruta encontrada:");
                        println!("   Distancia total: {} km", distancia);
                        println!("   Ruta: {}", ruta.join(" → "));
                    }
                    None => {
                        println!(" No se encontró una ruta entre las ciudades");
                    }
                }
            }
            "4" => {
                println!("\n[Procesando] Generando estructuras gráficas...");
                // Ejecutamos la función de exportación e imagen automatizada
                match funciones::exportar_dot(mapa_ciudades, "mapa_vías.dot") {
                    Ok(_) => {
                        println!(" ¡Operación Exitosa!");
                        println!("Se han guardado dos archivos en la raíz del proyecto:");
                        println!("  1. mapa_vías.dot (Código fuente del gráfico)");
                        println!("  2. mapa_vías.png (¡Tu imagen lista para ver!)");
                    }
                    Err(e) => {
                        println!(" Error al generar la imagen: {}", e);
                    }
                }
            }
            "5" => {
                println!("\n¡Gracias por utilizar el sistema académico de grafos! Saliendo...");
                break;
            }
            _ => {
                println!("Opción inválida. Por favor, digite un número entre 1 y 5.");
            }
        }
    }
}
