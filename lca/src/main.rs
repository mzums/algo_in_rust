use std::collections::HashMap;

struct Tree {
    graph: HashMap<usize, Vec<usize>>,
    depth: Vec<usize>,
    parent: Vec<Vec<Option<usize>>>,
}

impl Tree {
    fn new(size: usize) -> Self {
        let graph = HashMap::new();
        let depth = vec![0; size];
        let log = (size as f64).log2().ceil() as usize;
        let parent = vec![vec![None; log]; size];
        Tree { graph, depth, parent }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.graph.entry(u).or_insert_with(Vec::new).push(v);
        self.graph.entry(v).or_insert_with(Vec::new).push(u);
    }

    fn preprocess(&mut self, root: usize) {
        let mut visited = vec![false; self.depth.len()];
        self.dfs(root, None, 0, &mut visited);

        let log = self.parent[0].len();
        for j in 1..log {
            for i in 0..self.depth.len() {
                if let Some(parent_i) = self.parent[i][j - 1] {
                    self.parent[i][j] = self.parent[parent_i][j - 1];
                }
            }
        }
    }

    fn dfs(&mut self, node: usize, parent: Option<usize>, depth: usize, visited: &mut Vec<bool>) {
        visited[node] = true;
        self.depth[node] = depth;
        self.parent[node][0] = parent;

        if let Some(neighbors) = self.graph.get(&node) {
            let neighbors_copy = neighbors.clone(); // Kopiujemy sąsiadów, aby zwolnić immutable borrow
            for &neighbor in &neighbors_copy {
                if !visited[neighbor] {
                    self.dfs(neighbor, Some(node), depth + 1, visited);
                }
            }
        }
    }

    fn lca(&self, mut u: usize, mut v: usize) -> Option<usize> {
        if self.depth[u] < self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }

        let log = self.parent[0].len();
        for i in (0..log).rev() {
            if let Some(ancestor) = self.parent[u][i] {
                if self.depth[ancestor] >= self.depth[v] {
                    u = ancestor;
                }
            }
        }

        if u == v {
            return Some(u);
        }

        for i in (0..log).rev() {
            if self.parent[u][i] != self.parent[v][i] {
                u = self.parent[u][i].unwrap();
                v = self.parent[v][i].unwrap();
            }
        }

        self.parent[u][0]
    }
}

fn main() {
    let mut tree = Tree::new(7);

    tree.add_edge(0, 1);
    tree.add_edge(0, 2);
    tree.add_edge(1, 3);
    tree.add_edge(1, 4);
    tree.add_edge(2, 5);
    tree.add_edge(2, 6);

    tree.preprocess(0);

    println!("LCA of 3 and 4: {:?}", tree.lca(3, 4));
    println!("LCA of 3 and 6: {:?}", tree.lca(3, 6));
}
