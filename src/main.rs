use petgraph::graph::UnGraph;
mod funciones;
use crate::funciones::*;

fn main() {
    // 1. Inicializamos el grafo
    let mut mapa_ciudades = UnGraph::<String, u32>::new_undirected();

    println!("--- Creando Ciudades ---");
    // 2. Añadimos algunas ciudades
    añadir_ciudad(&mut mapa_ciudades, "San Salvador");
    añadir_ciudad(&mut mapa_ciudades, "Santa Ana");
    añadir_ciudad(&mut mapa_ciudades, "San Miguel");
    añadir_ciudad(&mut mapa_ciudades, "Sonsonate");

    println!("\n--- Creando Conexiones ---");
    // 3. Conectamos las ciudades con sus respectivas distancias (ejemplo)
    añadir_conexion(&mut mapa_ciudades, "San Salvador".to_string(), "Santa Ana".to_string(), 65);
    añadir_conexion(&mut mapa_ciudades, "San Salvador".to_string(), "San Miguel".to_string(), 138);
    añadir_conexion(&mut mapa_ciudades, "Santa Ana".to_string(), "Sonsonate".to_string(), 40);
    añadir_conexion(&mut mapa_ciudades, "Sonsonate".to_string(), "San Salvador".to_string(), 64);

    //Menu
    println!("\n--- Buscando Ruta Más Corta (BFS) ---");
    let origen = "Santa Ana".to_string(); // o "Sevilla"
    let destino = "San Miguel".to_string(); // o "Zaragoza"

    match ruta_mas_corta(&mapa_ciudades, origen.clone(), destino.clone()) {
        Some((_distancia_total, ruta)) => {
            // Unimos los nombres del vector con la flecha
            let ruta_formateada = ruta.join(" -> ");

            // Los saltos son la cantidad de ciudades en la ruta menos 1
            let saltos = ruta.len() - 1;

            // Imprimimos exactamente como lo pide el profesor
            println!("{} ({} saltos).", ruta_formateada, saltos);

            // Si el profesor también quiere ver la distancia en kilómetros,
            // puedes descomentar esta línea:
            // println!("Distancia total: {} km", _distancia_total);
        }
        None => {
            println!("No se pudo encontrar una ruta entre {} y {}.", origen, destino);
        }
    }



}