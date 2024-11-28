// #![deny(clippy::pedantic)]

use slab::Slab;
use std::cmp;
use std::ops::{Index, IndexMut};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Pointer(usize);

impl Pointer {
    #[inline]
    pub fn null() -> Pointer {
        Pointer(!0)
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        *self == Pointer::null()
    }
}

impl Index<Pointer> for RedBlackTree {
    type Output = Node;

    fn index(&self, index: Pointer) -> &Node {
        &self.slab[index.0]
    }
}

impl IndexMut<Pointer> for RedBlackTree {
    fn index_mut(&mut self, index: Pointer) -> &mut Node {
        &mut self.slab[index.0]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug)]
pub struct Node {
    pub value: u32,
    pub right: Pointer,
    pub left: Pointer,
    pub parent: Pointer,
    pub color: Color,
}

pub struct RedBlackTree {
    pub slab: Slab<Node>,
    pub root: Pointer,
}

impl RedBlackTree {
    pub fn new() -> Self {
        RedBlackTree {
            slab: Slab::new(),
            root: Pointer::null(),
        }
    }

    pub fn height(&self) -> u32 {
        self.height_below(self.root)
    }

    fn height_below(&self, node: Pointer) -> u32 {
        if node.is_null() {
            0
        } else {
            let left = self.height_below(self[node].left);
            let right = self.height_below(self[node].right);
            cmp::max(left, right) + 1
        }
    }

    pub fn red_count(&self) -> u32 {
        self.red_below(self.root)
    }

    fn red_below(&self, node: Pointer) -> u32 {
        let mut count = 0;
        if self[node].color == Color::Red {
            count += 1;
        }
        if !self[node].right.is_null() {
            count += self.red_below(self[node].right);
        }
        if !self[node].left.is_null() {
            count += self.red_below(self[node].left);
        }

        count
    }

    pub fn insert(&mut self, val: u32) {
        if self.root.is_null() {
            self.root = Pointer(self.slab.insert(Node {
                value: val,
                right: Pointer::null(),
                left: Pointer::null(),
                parent: Pointer::null(),
                color: Color::Black,
            }));
        } else {
            let new_node = self.insert_node(val, self.root);
            if !new_node.is_null() {
                self.insert_fixup(new_node);
            }
        }
    }

    fn insert_fixup(&mut self, node: Pointer) {
        let parent = self[node].parent;
        if self[node].parent.is_null() {
            return self.insert_case1(node);
        }

        if self[parent].color == Color::Black {
            return self.insert_case2(node);
        }

        let uncle = self.uncle(node);

        if uncle.is_null() {
            return self.insert_case4(node);
        }
        if self[uncle].color == Color::Black {
            return self.insert_case4(node);
        }

        return self.insert_case3(node);
    }

    fn insert_case1(&mut self, node: Pointer) {
        self[node].color = Color::Black;
    }

    fn insert_case2(&mut self, _node: Pointer) {
        return;
    }

    fn insert_case3(&mut self, node: Pointer) {
        let parent = self[node].parent;
        let uncle = self.uncle(node);
        let grandparent = self[parent].parent;

        self[parent].color = Color::Black;
        self[uncle].color = Color::Black;
        self[grandparent].color = Color::Red;

        self.insert_fixup(grandparent);
    }

    fn insert_case4(&mut self, node: Pointer) {
        let parent = self[node].parent;
        let grandparent = self[parent].parent;

        let parent_left = self[parent].left;
        let parent_right = self[parent].right;

        let grandparent_left = self[grandparent].left;
        let grandparent_right = self[grandparent].right;

        let mut n = node;

        if !parent_right.is_null()
            && !grandparent_left.is_null()
            && (self[n].value == self[parent_right].value)
            && (self[parent].value == self[grandparent_left].value)
        {
            self.rotate_left(parent);
            n = self[n].left;
        } else if !parent_left.is_null()
            && !grandparent_right.is_null()
            && (self[n].value == self[parent_left].value)
            && (self[parent].value == self[grandparent_right].value)
        {
            self.rotate_right(parent);
            n = self[n].right;
        }

        let parent = self[n].parent;
        let grandparent = self[parent].parent;

        let parent_left = self[parent].left;

        if !parent_left.is_null() && self[n].value == self[parent_left].value {
            self.rotate_right(grandparent);
        } else {
            self.rotate_left(grandparent);
        }

        self[parent].color = Color::Black;
        self[grandparent].color = Color::Red;
    }

    fn uncle(&self, node: Pointer) -> Pointer {
        let parent = self[node].parent;
        if parent.is_null() {
            return Pointer::null();
        }

        let grandparent = self[parent].parent;

        if grandparent.is_null() {
            return Pointer::null();
        }

        let grandparent_left = self[grandparent].left;
        let grandparent_right = self[grandparent].right;

        if grandparent_left.is_null() >> grandparent_right.is_null() {
            return Pointer::null();
        }

        if self[parent].value == self[grandparent_left].value {
            return grandparent_right;
        }

        return grandparent_left;
    }

    fn insert_node(&mut self, val: u32, node: Pointer) -> Pointer {
        let node_value = self[node].value;
        let left = self[node].left;
        let right = self[node].right;

        if val == node_value {
            return Pointer::null();
        } else if val > node_value {
            if right.is_null() {
                self[node].right = Pointer(self.slab.insert(Node {
                    value: val,
                    right: Pointer::null(),
                    left: Pointer::null(),
                    parent: node,
                    color: Color::Red,
                }));
                return self[node].right;
            } else {
                return self.insert_node(val, right);
            }
        } else if left.is_null() {
            self[node].left = Pointer(self.slab.insert(Node {
                value: val,
                right: Pointer::null(),
                left: Pointer::null(),
                parent: node,
                color: Color::Red,
            }));
            return self[node].left;
        } else {
            return self.insert_node(val, left);
        }
    }

    fn rotate_left(&mut self, current: Pointer) {
        let right = self[current].right;

        if right.is_null() {
            return;
        }

        let right_left = self[right].left;
        let parent = self[current].parent;

        self[current].right = right_left;

        if !right_left.is_null() {
            self[right_left].parent = current;
        }

        self[current].parent = right;
        self[right].left = current;

        self[right].parent = parent;

        if parent.is_null() {
            self.root = right;
