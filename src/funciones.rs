use petgraph::graph::UnGraph;

fn añadir_ciudad(grafo: &mut UnGraph<String, u32>, nombre: &str) -> NodoIndex {
   println!("Añadida ciudad correctamente");
   grafo.add_node(nombre.to_string())
}

fn añadir_conexion(
    grafo: &mut UnGraph<String, u32>,
    conexion_a: String,
    conexion_b: String,
    distancia:u32
) -> Option<EdgeIndex>{
    let mut nodo_a = None;
    let mut nodo_b= None;
    if validar(grafo) {
        
    
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

   match (nodo_a,nodo_b) {
       (Some(nodo_a),Some(nodo_b))=>{
        Some(grafo.add_edge(nodo_a, nodo_a, distancia))
       }
       _=> None,
   }}else {
       println!("El grafo debe tener al menos 2 nodos");
       return None;
   }
}

fn validar(grafo: &mut UnGraph<String, u32>)->bool{
  let contador= grafo.node_count();
  if contador>1 {
      return true;
  }
  else {
      return false;
  }
}
