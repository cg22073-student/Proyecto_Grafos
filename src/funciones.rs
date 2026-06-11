use petgraph::algo::astar;
use petgraph::dot::Dot;
use std::io::Result;
use std::process::Command;
use std::collections::{VecDeque,HashMap};
use petgraph::{graph::{UnGraph,NodeIndex,EdgeIndex}};

pub fn añadir_ciudad(grafo: &mut UnGraph<String, u32>, nombre: &str) -> NodeIndex {
    println!("Añadida ciudad correctamente");
    grafo.add_node(nombre.to_string())
}

pub fn añadir_conexion(
    grafo: &mut UnGraph<String, u32>,
    conexion_a: String,
    conexion_b: String,
    distancia: u32,
) -> Option<EdgeIndex> {
    let mut nodo_a = None;
    let mut nodo_b = None;
    let mut bandera = false;
    if validar(grafo) {
        if conexion_a != conexion_b {
            for nodo in grafo.node_indices() {
                if grafo[nodo] == conexion_a {
                    println!("Se encontro la ciudad:{}", conexion_a);
                    nodo_a = Some(nodo);
                    break;
                }
            }

            for nodo in grafo.node_indices() {
                if grafo[nodo] == conexion_b {
                    println!("Se encontro la ciudad:{}", conexion_b);
                    nodo_b = Some(nodo);
                    break;
                }
            }
            if let (Some(a), Some(b)) = (nodo_a, nodo_b) {
                bandera = grafo.find_edge(a, b).is_none();
            }
            if bandera {
                match (nodo_a, nodo_b) {
                    (Some(nodo_a), Some(nodo_b)) => Some(grafo.add_edge(nodo_a, nodo_b, distancia)),
                    _ => None,
                }
            } else {
                println!("Esa conexion ya existe");
                None
            }
        } else {
            println!("Deben ser dos ciudades distintas");
            None
        }
    } else {
        println!("El grafo debe tener al menos 2 nodos");
        return None;
    }
}

pub fn validar(grafo: &UnGraph<String, u32>) -> bool {
    if grafo.node_count() >= 2 { true } else { false }
}

pub fn ruta_mas_corta(
    grafo: &UnGraph<String, u32>,
    conexion_a: String,
    conexion_b: String,
) -> Option<(u32, Vec<String>)> {
    let mut nodo_inicio = None;
    let mut nodo_destino = None;

    if validar(grafo) {
        for nodo in grafo.node_indices() {
            if grafo[nodo] == conexion_a {
                nodo_inicio = Some(nodo);
            }
            if grafo[nodo] == conexion_b {
                nodo_destino = Some(nodo);
            }
        }

        if let (Some(inicio), Some(destino)) = (nodo_inicio, nodo_destino) {
            let mut cola = VecDeque::new();
            let mut predecesores: HashMap<NodeIndex, NodeIndex> = HashMap::new();

            cola.push_back(inicio);
            predecesores.insert(inicio, inicio);

            let mut encontrado = false;

            // --- INICIO DEL ALGORITMO BFS ---
            while let Some(nodo_actual) = cola.pop_front() {
                if nodo_actual == destino {
                    encontrado = true;
                    break;
                }

                for vecino in grafo.neighbors(nodo_actual) {
                    if !predecesores.contains_key(&vecino) {
                        predecesores.insert(vecino, nodo_actual);
                        cola.push_back(vecino);
                    }
                }
            }
            // --- FIN DEL ALGORITMO BFS ---

            if encontrado {
                let mut camino_indices = Vec::new();
                let mut actual = destino;

                while actual != inicio {
                    camino_indices.push(actual);
                    actual = *predecesores.get(&actual).unwrap();
                }
                camino_indices.push(inicio);
                camino_indices.reverse();

                let mut costo_total = 0;
                for i in 0..camino_indices.len() - 1 {
                    let nodo1 = camino_indices[i];
                    let nodo2 = camino_indices[i + 1];
                    if let Some(edge) = grafo.find_edge(nodo1, nodo2) {
                        costo_total += grafo[edge];
                    }
                }

                let camino_nombres: Vec<String> = camino_indices
                    .into_iter()
                    .map(|nodo| grafo[nodo].clone())
                    .collect();

                return Some((costo_total, camino_nombres));
            }
        }
        None
    } else {
        None
    }
}

pub fn exportar_dot(grafo: &UnGraph<String, u32>, nombre_archivo: &str) -> Result<String> {
    let dot = format!("{:?}", Dot::new(grafo));
    std::fs::write(nombre_archivo, &dot)?;
    println!("Archivo DOT generado exitosamente en: {}", nombre_archivo);

    let nombre_imagen = nombre_archivo.replace(".dot", ".png");

    let output = Command::new("dot")
        .arg("-Tpng")
        .arg(nombre_archivo)
        .arg("-o")
        .arg(&nombre_imagen)
        .output();

    match output {
        Ok(out) if out.status.success() => {
            println!("Imagen generada exitosamente en: {}", nombre_imagen);
        }
        Ok(out) => {
            let error = String::from_utf8_lossy(&out.stderr);
            eprintln!("Se intentó generar la imagen, pero Graphviz reportó un error:\n{}", error);
        }
        Err(e) => {
            println!("Nota: No se pudo generar la imagen PNG porque Graphviz ('dot') no está instalado o no se encuentra en el PATH.\nDetalle: {}", e);
        }
    }

    Ok(dot)
}
