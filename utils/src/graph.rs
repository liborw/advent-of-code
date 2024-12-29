use std::{collections::{HashMap, HashSet}, hash::Hash};


pub fn maximal_cliques<T>(graph: &HashMap<T, HashSet<T>>) -> Vec<Vec<T>>
where T: Clone + Eq + Hash + Ord
{
    // Initialize R, P, X
    let r: HashSet<_> = HashSet::new();
    let mut p: HashSet<_> = graph.keys().cloned().collect();
    let mut x: HashSet<_> = HashSet::new();

    // Collect cliques
    let mut cliques: Vec<Vec<_>> = Vec::new();
    bron_kerbosch_algo(&r, &mut p, &mut x, graph, &mut cliques);
    cliques
}


/// Find all maximal cliques
pub fn bron_kerbosch_algo<T>(
    r: &HashSet<T>,
    p: &mut HashSet<T>,
    x: &mut HashSet<T>,
    g: &HashMap<T, HashSet<T>>,
    cliques: &mut Vec<Vec<T>>
) where T: Clone + Eq + Hash + Ord
{
    if p.is_empty() && x.is_empty() {
        if r.len() > 2 {
            let mut clique: Vec<T> = r.iter().cloned().collect();
            clique.sort();
            cliques.push(clique);
        }
        return;
    }

    // Choose a pivot with the maximum degree in P ∪ X
    let pivot = p
        .union(x)
        .max_by_key(|v| g.get(*v).map_or(0, |neighbors| neighbors.len()))
        .cloned();

    if let Some(pivot_vertex) = pivot {
        let neighbors = g.get(&pivot_vertex).cloned().unwrap_or_default();
        let candidates: Vec<T> = p.difference(&neighbors).cloned().collect();

        for v in candidates {
            // New R is R ∪ {v}
            let mut new_r = r.clone();
            new_r.insert(v.clone());

            // New P is P ∩ N(v)
            let neighbors_v = g.get(&v).cloned().unwrap_or_default();
            let mut new_p = p.intersection(&neighbors_v).cloned().collect::<HashSet<T>>();

            // New X is X ∩ N(v)
            let mut new_x = x.intersection(&neighbors_v).cloned().collect::<HashSet<T>>();

            // Recursive call
            bron_kerbosch_algo(&new_r, &mut new_p, &mut new_x, g, cliques);

            // Move v from P to X
            p.remove(&v);
            x.insert(v);
        }
    }
}
