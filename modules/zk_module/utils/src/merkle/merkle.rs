pub type LeafType = u64;
const L_ZERO: LeafType = 0;

/// Implements a Merkle tree data structures with methods for
/// building the tree, computing the Merkle root, generating proof paths, and verifying
/// data integrity using Merkle proofs.

#[allow(warnings)]
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MerkleTree {
    tree: HashMap<usize, Vec<LeafType>>,
    depth: usize,
    pub root: LeafType,
}

impl Default for MerkleTree {
    fn default() -> Self {
        Self::new()
    }
}

impl MerkleTree {
    pub fn new() -> Self {
        let mut tree: HashMap<usize, Vec<LeafType>> = HashMap::new();
        tree.insert(0, vec![]);
        Self {
            tree,
            depth: 0,
            root: L_ZERO,
        }
    }

    pub fn get_leaves(&self) -> Vec<LeafType> {
        self.tree.get(&0).unwrap().to_vec()
    }

    pub fn build(&mut self, values: &Vec<LeafType>) {
        let depth = (values.len() as f32).log2().floor() as usize + 2;
        let mut tree: HashMap<usize, Vec<LeafType>> = HashMap::new();
        // insert data into tree
        tree.insert(0, values.clone());
        if tree[&0].len() % 2 == 1 {
            tree.get_mut(&0).unwrap().push(L_ZERO);
        }
        for i in 1..depth {
            let len = tree.get(&(i - 1)).unwrap().len();
            let mut this_layer = vec![];
            let mut j = 0;
            while j < len {
                let f = tree.get(&(i - 1)).unwrap().get(j).unwrap();
                // If the length of the array is odd at each level, the last element is added again to the end of the array
                let s = tree.get(&(i - 1)).unwrap().get(j + 1).unwrap_or(f);
                let hash_node = Self::hash_node(*f, *s);
                this_layer.push(hash_node);
                j += 2;
            }
            if this_layer.len() % 2 == 1 && i != depth - 1 {
                this_layer.push(L_ZERO);
            }
            tree.insert(i, this_layer);
        }
        let root = *tree.get(&(depth - 1)).unwrap().first().unwrap();
        self.tree = tree;
        self.depth = depth;
        self.root = root;
    }

    #[inline]
    fn insert_leaf(leaf_layer: &mut Vec<LeafType>, new_value: LeafType) {
        // Insert the new data into the leaf layer
        if let Some(0) = leaf_layer.last() {
            leaf_layer.pop();
            leaf_layer.push(new_value);
        } else {
            leaf_layer.push(new_value);
            leaf_layer.push(L_ZERO);
        }
    }

    pub fn insert(&mut self, new_record: LeafType) {
        let leaf_layer = self.tree.entry(0).or_insert_with(|| vec![new_record]);
        let mut new_depth = (1.0 + leaf_layer.len() as f32).log2().ceil() as usize;
        if !leaf_layer.len().is_power_of_two() {
            new_depth += 1;
        }
        // Insert the new data into the leaf layer
        Self::insert_leaf(leaf_layer, new_record);
        // Update the tree
        for i in 1..new_depth {
            let downer_layer = self.tree.get(&(i - 1)).unwrap();
            let mut this_layer = vec![];
            let mut j = 0;
            while j < downer_layer.len() {
                let element_v1 = downer_layer[j];
                let element_v2 = if j + 1 < downer_layer.len() {
                    downer_layer[j + 1]
                } else {
                    L_ZERO
                };
                let node = Self::hash_node(element_v1, element_v2);
                this_layer.push(node);
                j += 2;
            }
            if this_layer.len() % 2 == 1 && i != new_depth - 1 {
                this_layer.push(L_ZERO);
            }
            self.tree.insert(i, this_layer);
        }
        // Update the root and depth
        let new_root = *self.tree.get(&(new_depth - 1)).unwrap().first().unwrap();
        self.depth = new_depth;
        self.root = new_root;
    }

    /// Returns a tuple (path, indices)
    /// Indices: 0 indicates the right sibling and 1 indicates the left sibling
    pub fn proof_path(&self, leaf: LeafType) -> Option<(Vec<LeafType>, Vec<LeafType>)> {
        let mut index = match self.tree.get(&0).unwrap().iter().position(|&r| r == leaf) {
            Some(idx) => idx,
            None => return None,
        };
        let mut path = vec![];
        let mut indices = vec![];
        let mut node;
        for i in 0..self.depth - 1 {
            if index % 2 == 0 {
                indices.push(L_ZERO);
                node = *self.tree.get(&i)?.get(index + 1).unwrap();
            } else {
                indices.push(1);
                node = *self.tree.get(&i)?.get(index - 1).unwrap();
            }
            index = (index as f32 / 2.0).floor() as usize;
            path.push(node);
        }
        Some((path, indices))
    }

    fn hasher_u64(value: LeafType) -> LeafType {
        let mut hasher = std::hash::DefaultHasher::new();
        std::hash::Hash::hash(&value, &mut hasher);
        std::hash::Hasher::finish(&hasher)
    }

    pub fn hash_node(a: LeafType, b: LeafType) -> LeafType {
        let hashed_val1 = Self::hasher_u64(a);
        let hashed_val2 = Self::hasher_u64(b);
        let combined_hash = hashed_val1.wrapping_add(hashed_val2);
        Self::hasher_u64(combined_hash)
    }

    pub fn print(&self) {
        for i in 0..self.tree.len() {
            println!("{}: ", i);
            println!("{:#?}", self.tree.get(&i).unwrap())
        }
        println!("root: {:?}", self.root);
    }
}

#[allow(dead_code)]
pub fn compute_merkle_root(leaf: &LeafType, elements: &Vec<LeafType>, indices: &Vec<LeafType>) -> LeafType {
    let k = elements.len();
    let mut digest = *leaf;
    let mut message: [LeafType; 2];
    for i in 0..k {
        if indices[i] == L_ZERO {
            message = [digest, elements[i]];
        } else {
            message = [elements[i], digest];
        }
        digest = MerkleTree::hash_node(message[0], message[1]);
    }
    digest
}

#[allow(dead_code)]
pub fn verify(tree: &MerkleTree, leaf: &LeafType, elements: &Vec<LeafType>, indices: &Vec<LeafType>) -> bool {
    tree.root == compute_merkle_root(leaf, elements, indices)
}

// #[cfg(test)]
// mod test {
//     use crate::mongo_database::{mongo_database::MongoDB, records::UserRecord};
//     use merkle_tree::{verify, MerkleTree};
//     use mongodb::bson::doc;

//     use super::*;

//     const MONGO_URI: &str = "mongodb://localhost:27017/";

//     #[test]
//     fn merkle_verify_path() {
//         let mongodb = MongoDB::new(MONGO_URI, "DB1", "Col1");
//         mongodb.drop().unwrap();

//         mongodb
//             .collection_json
//             .insert_one(UserRecord::new("alireza0".to_string(), 12345), None)
//             .unwrap();

//         let mut merkle = MerkleTree::new();

//         client::insert_random_records(&mut merkle, &mongodb.collection, 10);

//         let query_result = mongodb.query(doc! {"name": "alireza0"}).unwrap();

//         assert!(client::verify_all(&merkle, query_result));
//     }

//     #[test]
//     fn merkle_verify_path_failure() {
//         let mongodb = MongoDB::new(MONGO_URI, "DB1", "Col1");
//         mongodb.drop().unwrap();

//         let mut merkle = MerkleTree::new();

//         client::insert_random_records(&mut merkle, &mongodb.collection, 10);

//         let query_result = mongodb.query(doc! {"name": "NO_RECORD"}).unwrap();

//         assert!(!client::verify_all(&merkle, query_result));
//     }

//     #[test]
//     fn merkle_duplicate_inputs() {
//         let mongodb = MongoDB::new(MONGO_URI, "DB1", "Col1");
//         mongodb.drop().unwrap();

//         mongodb
//             .collection
//             .insert_one(UserRecord::new("alireza1".to_string(), 12345), None)
//             .unwrap();
//         mongodb
//             .collection
//             .insert_one(UserRecord::new("alireza1".to_string(), 12345), None)
//             .unwrap();
//         mongodb
//             .collection
//             .insert_one(UserRecord::new("alireza1".to_string(), 12345), None)
//             .unwrap();

//         let mut merkle = MerkleTree::new();

//         client::insert_random_records(&mut merkle, &mongodb.collection, 10);

//         let query_result = mongodb.query(doc! {"name": "alireza1"}).unwrap();

//         assert!(client::verify_all(&merkle, query_result));
//     }

//     #[test]
//     fn verify_leaves() {
//         let leaves = vec![1, 2, 3, 4, 5];

//         let mut merkle_tree = merkle_tree::MerkleTree::new();
//         merkle_tree.build(&leaves);

//         for leaf in merkle_tree.get_leaves() {
//             let proof_path = merkle_tree.proof_path(leaf).unwrap();
//             assert!(verify(&merkle_tree, &leaf, &proof_path.0, &proof_path.1));
//         }
//     }

//     #[test]
//     fn insert_new_leaf() {
//         let leaves = vec![1, 2, 3, 4, 5];

//         let mut merkle_tree = merkle_tree::MerkleTree::new();
//         merkle_tree.build(&leaves.clone());

//         merkle_tree.insert(6);
//         merkle_tree.insert(7);

//         for leaf in merkle_tree.get_leaves() {
//             let proof_path = merkle_tree.proof_path(leaf).unwrap();
//             assert!(verify(&merkle_tree, &leaf, &proof_path.0, &proof_path.1));
//         }

//         merkle_tree.insert(8);

//         for leaf in merkle_tree.get_leaves() {
//             let proof_path = merkle_tree.proof_path(leaf).unwrap();
//             assert!(verify(&merkle_tree, &leaf, &proof_path.0, &proof_path.1));
//         }

//         merkle_tree.insert(8);
//         merkle_tree.insert(9);
//         merkle_tree.insert(10);

//         for leaf in merkle_tree.get_leaves() {
//             let proof_path = merkle_tree.proof_path(leaf).unwrap();
//             assert!(verify(&merkle_tree, &leaf, &proof_path.0, &proof_path.1));
//         }
//     }


//     #[test]
//     fn insert_leafs() {
//         let mut merkle_tree = merkle_tree::MerkleTree::new();

//         merkle_tree.insert(6);
//         merkle_tree.insert(7);

//         for leaf in merkle_tree.get_leaves() {
//             let proof_path = merkle_tree.proof_path(leaf).unwrap();
//             assert!(verify(&merkle_tree, &leaf, &proof_path.0, &proof_path.1));
//         }

//         merkle_tree.insert(8);

//         for leaf in merkle_tree.get_leaves() {
//             let proof_path = merkle_tree.proof_path(leaf).unwrap();
//             assert!(verify(&merkle_tree, &leaf, &proof_path.0, &proof_path.1));
//         }

//         merkle_tree.insert(8);
//         merkle_tree.insert(9);
//         merkle_tree.insert(10);

//         for leaf in merkle_tree.get_leaves() {
//             let proof_path = merkle_tree.proof_path(leaf).unwrap();
//             assert!(verify(&merkle_tree, &leaf, &proof_path.0, &proof_path.1));
//         }
//     }


//     #[test]
//     fn insert_leafs_inloop() {
//         let mut merkle_tree = merkle_tree::MerkleTree::new();

//         for i in 1..=7 {
//             merkle_tree.insert(i);
//         }


//         for leaf in 1..=7 {
//             let proof_path = merkle_tree.proof_path(leaf).unwrap();
//             assert!(verify(&merkle_tree, &leaf, &proof_path.0, &proof_path.1));
//         }

//         merkle_tree.insert(8);

        
//         for leaf in 1..=8 {
//             let proof_path = merkle_tree.proof_path(leaf).unwrap();
//             assert!(verify(&merkle_tree, &leaf, &proof_path.0, &proof_path.1));
//         }
//     }

//     // merkle_huge
//     // merkle_huge_insert    
// }
