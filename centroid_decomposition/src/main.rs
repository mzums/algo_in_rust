struct CentroidDecomposition {
    tree: Vec<Vec<usize>>,
    size: Vec<usize>,
    visited: Vec<bool>,
    parent: Vec<Option<usize>>,
}

impl CentroidDecomposition {
    pub fn new(n: usize) -> Self {
        CentroidDecomposition {
            tree: vec![vec![]; n],
            size: vec![0; n],
            visited: vec![false; n],
            parent: vec![None; n],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.tree[u].push(v);
        self.tree[v].push(u);
    }

    fn calculate_subtree_sizes(&mut self, node: usize, parent: Option<usize>) -> usize {
        self.size[node] = 1;
        for neighbor in self.tree[node].clone() {
            if Some(neighbor) != parent && !self.visited[neighbor] {
                self.size[node] += self.calculate_subtree_sizes(neighbor, Some(node));
            }
        }
        self.size[node]
    }

    fn find_centroid(&mut self, node: usize, parent: Option<usize>, total_size: usize) -> usize {
        for neighbor in self.tree[node].clone() {
            if Some(neighbor) != parent
                && !self.visited[neighbor]
                && self.size[neighbor] > total_size / 2
            {
                return self.find_centroid(neighbor, Some(node), total_size);
            }
        }
        node
    }

    fn decompose(&mut self, start: usize, parent: Option<usize>) {
        let total_size = self.calculate_subtree_sizes(start, None);
        let centroid = self.find_centroid(start, None, total_size);

        self.visited[centroid] = true;
        self.parent[centroid] = parent;

        for neighbor in self.tree[centroid].clone() {
            if !self.visited[neighbor] {
                self.decompose(neighbor, Some(centroid));
            }
        }
    }

    pub fn centroid_decomposition(&mut self) {
        self.decompose(0, None);
    }


    pub fn print_tree(&self) {
        println!("Drzewo centroidów:");
        for (node, parent) in self.parent.iter().enumerate() {
            match parent {
                Some(p) => println!("Node {} -> Parent: {}", node, p),
                None => println!("Node {} -> No parent (root)", node),
            }
        }
    }
}

fn main() {
    let mut cd = CentroidDecomposition::new(7);

    cd.add_edge(0, 1);
    cd.add_edge(0, 2);
    cd.add_edge(1, 3);
    cd.add_edge(1, 4);
    cd.add_edge(2, 5);
    cd.add_edge(2, 6);

    cd.centroid_decomposition();

    cd.print_tree();
}
