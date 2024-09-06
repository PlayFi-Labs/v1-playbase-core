
#![no_main]
// this function verifies a Merkle proof for a given leaf value.
// - proof_path variable: This is a tuple containing two vectors:
//   - proof_path.0: The sibling hashes along the path from the leaf to the root.
//   - proof_path.1: A vector of 0s and 1s indicating whether the sibling hash is 
//     to the left (1) or right (0) of the current node.
//pub use jolt::provable;
//
//fn merkle(root: u64, proof_path: (Vec<u64>, Vec<u64>), leaf: u64) -> bool {
//    let k = proof_path.0.len();
//    let mut digest: u64 = leaf;
//    let mut message: [u64; 2];
//
//    for i in 0..k {
//        // indices: 0 indicates the right sibling and 1 indicates the left sibling
//        if proof_path.1[i] == 0 {
//            message = [digest, proof_path.0[i]];
//        } else {
//            message = [proof_path.0[i], digest];
//        }
//
//        digest = hash_node(message[0], message[1]);
//    }
//
//    root == digest
//}
//
//fn hasher_u64(value: u64) -> u64 {
//    let mut hasher = std::hash::DefaultHasher::new();
//    std::hash::Hash::hash(&value, &mut hasher);
//    std::hash::Hasher::finish(&hasher)
//}
//
//pub fn hash_node(a: u64, b: u64) -> u64 {
//    let hashed_val1 = hasher_u64(a);
//    let hashed_val2 = hasher_u64(b);
//
//    let combined_hash = hashed_val1.wrapping_add(hashed_val2); 
//
//    hasher_u64(combined_hash)
//}