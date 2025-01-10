use std::collections::HashMap;

fn find_centroid(tree: &HashMap<usize, Vec<usize>>, n: usize) -> usize {
    let mut subtree_sizes = vec![0; n];

    fn calculate_subtree_sizes(
        node: usize,
        parent: Option<usize>,
        tree: &HashMap<usize, Vec<usize>>,
        subtree_sizes: &mut Vec<usize>,
    ) -> usize {
        subtree_sizes[node] = 1;
        for &neighbor in &tree[&node] {
            if Some(neighbor) != parent {
                subtree_sizes[node] += calculate_subtree_sizes(neighbor, Some(node), tree, subtree_sizes);
            }
        }
        subtree_sizes[node]
    }

    fn find_centroid_dfs(
        node: usize,
        parent: Option<usize>,
        tree: &HashMap<usize, Vec<usize>>,
        subtree_sizes: &Vec<usize>,
        n: usize,
    ) -> usize {
        for &neighbor in &tree[&node] {
            if Some(neighbor) != parent && subtree_sizes[neighbor] > n / 2 {
                return find_centroid_dfs(neighbor, Some(node), tree, subtree_sizes, n);
            }
        }
        node
    }

    calculate_subtree_sizes(0, None, tree, &mut subtree_sizes);
    find_centroid_dfs(0, None, tree, &subtree_sizes, n)
}

fn main() {
    let mut tree: HashMap<usize, Vec<usize>> = HashMap::new();

    tree.insert(0, vec![1, 2]);
    tree.insert(1, vec![0, 3, 4]);
    tree.insert(2, vec![0]);
    tree.insert(3, vec![1]);
    tree.insert(4, vec![1]);

    let n = tree.len();
    let centroid = find_centroid(&tree, n);

    println!("Centroid: {}", centroid);
}
