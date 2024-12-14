struct SegmentTree {
    n: usize,
    tree: Vec<i64>,
    lazy: Vec<i64>,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        let n = size.next_power_of_two();
        SegmentTree {
            n,
            tree: vec![0; 2 * n],
            lazy: vec![0; 2 * n],
        }
    }

    fn propagate(&mut self, node: usize, node_low: usize, node_high: usize) {
        if self.lazy[node] != 0 {
            self.tree[node] += self.lazy[node] * (node_high - node_low + 1) as i64;
            if node_low != node_high {
                self.lazy[2 * node] += self.lazy[node];
                self.lazy[2 * node + 1] += self.lazy[node];
            }
            self.lazy[node] = 0;
        }
    }

    fn update_range(&mut self, update_low: usize, update_high: usize, value: i64, node: usize, node_low: usize, node_high: usize) {
        self.propagate(node, node_low, node_high);
        
        if update_low > node_high || update_high < node_low {
            return;
        }

        if update_low <= node_low && node_high <= update_high {
            self.lazy[node] += value;
            self.propagate(node, node_low, node_high);
            return;
        }

        let mid = (node_low + node_high) / 2;
        self.update_range(update_low, update_high, value, 2 * node, node_low, mid);
        self.update_range(update_low, update_high, value, 2 * node + 1, mid + 1, node_high);

        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
    }

    fn query_range(&mut self, query_low: usize, query_high: usize, node: usize, node_low: usize, node_high: usize) -> i64 {
        self.propagate(node, node_low, node_high);

        if query_low > node_high || query_high < node_low {
            return 0;
        }

        if query_low <= node_low && node_high <= query_high {
            return self.tree[node];
        }

        let mid = (node_low + node_high) / 2;
        let left_sum = self.query_range(query_low, query_high, 2 * node, node_low, mid);
        let right_sum = self.query_range(query_low, query_high, 2 * node + 1, mid + 1, node_high);

        left_sum + right_sum
    }

    fn update(&mut self, low: usize, high: usize, value: i64) {
        self.update_range(low, high, value, 1, 0, self.n - 1);
    }

    fn query(&mut self, low: usize, high: usize) -> i64 {
        self.query_range(low, high, 1, 0, self.n - 1)
    }
}

fn main() {
    let mut seg_tree = SegmentTree::new(8);

    seg_tree.update(1, 4, 2);

    seg_tree.update(3, 6, 3);

    let result = seg_tree.query(2, 6);
    println!("Sum [2, 5]: {}", result);
}


