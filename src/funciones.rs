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

