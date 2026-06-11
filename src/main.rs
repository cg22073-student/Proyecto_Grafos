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

   
}