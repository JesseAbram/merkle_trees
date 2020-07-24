use hex::encode;
use sha3::{Digest, Sha3_256};

fn main() {
    // data to be used as leaves
    let leaves = get_data();
    // Input the index of the word you want to create the proof for
    let proof_index = 0;
    let first_hashed_leaves = first_hashing(&leaves, &proof_index);
    let mut root = first_hashed_leaves.hashed_leaves.clone();
    let mut proof = Vec::new();
    proof.push(first_hashed_leaves.adjacent_hash.clone());
    let mut adjacent_hash = first_hashed_leaves.adjacent_hash;
    while root.len() > 1 {
        let return_data = reduce_merkle_branches(root, adjacent_hash);
        root = return_data.row;
        adjacent_hash = return_data.adjacent_hash.clone();
        proof.push(return_data.adjacent_hash)
    }

    let is_proved = check_proof(&proof, &proof_index, &leaves[proof_index]);

    println!(
        "proof {:?}, leaf_index {}, my_word {}",
        &proof, &proof_index, leaves[proof_index]
    );
    println!("I am root {:?}", root);
    println!("is proved? {:?}", is_proved);
}
/// return type for reduce_merkle_branches function
struct MerkleBranchReturn {
    row: Vec<String>,
    adjacent_hash: String,
}
/// takes an array and hashes the adjacent hashes together
/// returns the adjacent hash for the proof
/// handles uneven tree size by hashing the last node twice if needed
fn reduce_merkle_branches(nodes: Vec<String>, current_adjacent_hash: String) -> MerkleBranchReturn {
    let mut row = Vec::with_capacity((nodes.len() + 1) / 2);
    let mut i = 0;
    // loops through the nodes and hashes adjacent nodes returning the next layer of the tree
    while i < nodes.len() {
        // handles uneven tree by adding the same node twice
        if nodes.len() - row.len() * 2 != 1 {
            row.push(hash_nodes(nodes[i].clone(), nodes[i + 1].clone()));
        } else {
            row.push(hash_nodes(nodes[i].clone(), nodes[i].clone()));
        }
        i += 2;
    }
    // for the proof, finds where the current adjacent hash is
    let adjacent_hash_index = nodes
        .iter()
        .position(|r| r == &current_adjacent_hash)
        .unwrap();
    // divides it by two to represent the position in the next layer of the tree
    let next_index_level = adjacent_hash_index / 2;
    // if this is the last level exists here with the root
    if row.len() < 2 {
        return MerkleBranchReturn {
            adjacent_hash: row[0].clone(),
            row,
        };
    }
    // if the index position is even get the node to the right, odd get the node to the left
    let adjacent_hash = if next_index_level % 2 == 0 {
        row[next_index_level + 1].clone()
    } else {
        row[next_index_level - 1].clone()
    };
    MerkleBranchReturn { row, adjacent_hash }
}
/// hashes a left and right node together returns hash
fn hash_nodes(left: String, right: String) -> String {
    let mut hasher = Sha3_256::new();
    let concat = left + &right;
    hasher.update(concat);
    encode(hasher.finalize())
}
/// return type for first_hashing function
struct FirstHashReturn {
    hashed_leaves: Vec<String>,
    adjacent_hash: String,
}
/// takes a data set of vec of strings and hashes them
/// returns the hashes of all the input data and the adjacent hash of the leaf to prove
fn first_hashing(leaves: &Vec<String>, index: &usize) -> FirstHashReturn {
    let mut i = 0;
    // hashes all the leaves, appends an index to force unique hash (almost like a nonce)
    let hashed_leaves: Vec<String> = leaves
        .iter()
        .map(|x| {
            let unique_word = x.clone() + (&i.to_string());
            let mut hasher = Sha3_256::new();
            hasher.update(unique_word);
            i += 1;
            encode(hasher.finalize())
        })
        .collect();
    // find adjacent hash, if even it is the right if odd it is the left
    let adjacent_hash = if index % 2 == 0 {
        hashed_leaves[index + 1].clone()
    } else {
        hashed_leaves[index - 1].clone()
    };
    FirstHashReturn {
        adjacent_hash,
        hashed_leaves,
    }
}
/// takes in proof, recreates the tree based on the proof
/// returns true if the root matches
fn check_proof(nodes: &Vec<String>, index: &usize, word: &String) -> bool {
    // hash the piece of data to be checked
    let mut hasher = Sha3_256::new();
    let unique_word = word.clone() + &index.to_string();
    hasher.update(unique_word);
    let hashed_word = encode(hasher.finalize());

    // determines based on position in tree if node is on the left or right
    // hashes the first level of the tree
    let first_level = if index % 2 == 0 {
        hash_nodes(hashed_word, nodes[0].clone())
    } else {
        hash_nodes(nodes[0].clone(), hashed_word)
    };

    let mut i = 1;
    let mut current_hash = first_level;
    let mut new_index = *index;
    // hashes the next levels to get the root
    while i < nodes.len() - 1 {
        let return_data = reduce_proof(nodes[i].clone(), current_hash, new_index);
        current_hash = return_data.hashed_leaves;
        new_index = return_data.new_index_position;

        i += 1;
    }
    // if the roots match return true
    current_hash == nodes[nodes.len() - 1]
}

struct ReduceProofReturn {
    hashed_leaves: String,
    new_index_position: usize,
}
/// takes two nodes of the tree and hashes them to create the next level of the tree
fn reduce_proof(next_hash: String, current_hash: String, leaf_index: usize) -> ReduceProofReturn {
    // the next position in the tree is the current position divided by two and rounded down but u64 handles
    let new_index_position = leaf_index / 2;
    let hashed_leaves;
    // determines based on position in tree if node is on the left or right
    if new_index_position % 2 == 0 {
        hashed_leaves = hash_nodes(current_hash, next_hash);
    } else {
        hashed_leaves = hash_nodes(next_hash, current_hash);
    }
    ReduceProofReturn {
        hashed_leaves,
        new_index_position: new_index_position,
    }
}
/// returns data to be put in tree
fn get_data() -> Vec<String> {
    vec![
        "like".into(),
        "this".into(),
        "that".into(),
        "and".into(),
        "this".into(),
        "and".into(),
        "that".into(),
        "so".into(),
        "just".into(),
        "chill".into(),
        "till".into(),
        "the".into(),
        "next".into(),
        "episode".into(),
        "one".into(),
        "two".into()
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_hashes_values() {
        let test_data = vec!["like".into(), "this".into()];
        let hashed_test_data = first_hashing(&test_data, &0);
        let expected_result = vec![
            "1bd79bdf98d0505586d38900bca78f0455da9580e95bf86933e7854ef1cf7eeb",
            "003285cad89f49174a3ecba65402ab49bb72e220047ccdee8bec9ea6fd28312c",
        ];
        assert_eq!(expected_result, hashed_test_data.hashed_leaves);
        assert_eq!(expected_result[1], hashed_test_data.adjacent_hash);
    }

    #[test]
    fn it_calculates_root() {
        let expected_result =
            vec!["922ba1684fd93c2fd24d0f79ddb90acc562e3da0df555eb89ad85ef595b1cb5c"];
        let first_hashed_leaves = first_hashing(&get_data(), &0);
        let mut root = first_hashed_leaves.hashed_leaves;
        let mut adjacent_hash = first_hashed_leaves.adjacent_hash;
        while root.len() > 1 {
            let return_data = reduce_merkle_branches(root, adjacent_hash);
            root = return_data.row;
            adjacent_hash = return_data.adjacent_hash;
        }
        assert_eq!(root, expected_result)
    }

    #[test]
    fn it_hashes_nodes_correctly() {
        let expected_result =
            "0f133f89141a9e62f8324dafa8d822501236c92db4793438a9ea2ebf6919dd2b".to_string();
        let left = "1bd79bdf98d0505586d38900bca78f0455da9580e95bf86933e7854ef1cf7eeb".to_string();
        let right = "003285cad89f49174a3ecba65402ab49bb72e220047ccdee8bec9ea6fd28312c".to_string();
        let hashed_node = hash_nodes(left, right);
        assert_eq!(hashed_node, expected_result);
    }

    #[test]
    fn it_calculates_correct_proof_to_true() {
        let proof = vec![
            "003285cad89f49174a3ecba65402ab49bb72e220047ccdee8bec9ea6fd28312c".to_string(),
            "2d2371e7f09b3451518e88df1c75aba5658cf606d3b9e4abf5f0a2d29c710434".to_string(),
            "1e37af00d38cd7217c5a2ecc8df9c8239e6b784021ee22d5f40057039d51c1af".to_string(),
            "e29d44c47b8dd9dfd26abb828410b43fe2bc920181657cf1598e3a9a72cce8ca".to_string(),
            "922ba1684fd93c2fd24d0f79ddb90acc562e3da0df555eb89ad85ef595b1cb5c".to_string(),
        ];
        let proof_index = 0;
        let my_word = "like".to_string();
        let is_correct = check_proof(&proof, &proof_index, &my_word);
        assert_eq!(is_correct, true);
    }
    #[test]
    fn it_calculates_incorrect_proof_to_false_with_wrong_word() {
        let proof = vec![
            "003285cad89f49174a3ecba65402ab49bb72e220047ccdee8bec9ea6fd28312c".to_string(),
            "2d2371e7f09b3451518e88df1c75aba5658cf606d3b9e4abf5f0a2d29c710434".to_string(),
            "1e37af00d38cd7217c5a2ecc8df9c8239e6b784021ee22d5f40057039d51c1af".to_string(),
            "e29d44c47b8dd9dfd26abb828410b43fe2bc920181657cf1598e3a9a72cce8ca".to_string(),
            "922ba1684fd93c2fd24d0f79ddb90acc562e3da0df555eb89ad85ef595b1cb5c".to_string(),
        ];
        let proof_index = 0;
        let my_word = "this".to_string();
        let is_correct = check_proof(&proof, &proof_index, &my_word);
        assert_eq!(is_correct, false);
    }
    #[test]
    fn it_calculates_incorrect_proof_to_false_with_wrong_proof() {
        let proof = vec![
            "003285cad89f49174a3ecba65402ab49bb72e220047ccdee8bec9ea6fd28312a".to_string(),
            "2d2371e7f09b3451518e88df1c75aba5658cf606d3b9e4abf5f0a2d29c710434".to_string(),
            "1e37af00d38cd7217c5a2ecc8df9c8239e6b784021ee22d5f40057039d51c1af".to_string(),
            "e29d44c47b8dd9dfd26abb828410b43fe2bc920181657cf1598e3a9a72cce8ca".to_string(),
            "922ba1684fd93c2fd24d0f79ddb90acc562e3da0df555eb89ad85ef595b1cb5c".to_string(),
        ];
        let proof_index = 0;
        let my_word = "like".to_string();
        let is_correct = check_proof(&proof, &proof_index, &my_word);
        assert_eq!(is_correct, false);
    }
    #[test]
    fn it_calculates_root_for_non_power_of_two_tree() {
        let data = vec![
            "like".into(),
            "this".into(),
            "that".into(),
            "and".into(),
            "this".into(),
            "and".into(),
            "that".into(),
            "so".into(),
            "just".into(),
            "chill".into(),
            "till".into(),
            "the".into(),
            "next".into(),
            "episode".into(),
            "one".into(),
        ];
        let expected_result = vec!["dfbf4654354900cd2580068b8baaf10e3a78ba7fff3e594eeef71ff34435c1f6"];
        let first_hashed_leaves = first_hashing(&data, &0);
        let mut root = first_hashed_leaves.hashed_leaves;
        let mut adjacent_hash = first_hashed_leaves.adjacent_hash;
        while root.len() > 1 {
            let return_data = reduce_merkle_branches(root, adjacent_hash);
            root = return_data.row;
            adjacent_hash = return_data.adjacent_hash;
        }
        assert_eq!(root, expected_result)
    }
    #[test]
    fn it_calculates_correct_proof_to_with_uneven_leaves() {
        let proof = vec![
            "003285cad89f49174a3ecba65402ab49bb72e220047ccdee8bec9ea6fd28312c".to_string(),
            "2d2371e7f09b3451518e88df1c75aba5658cf606d3b9e4abf5f0a2d29c710434".to_string(),
            "1e37af00d38cd7217c5a2ecc8df9c8239e6b784021ee22d5f40057039d51c1af".to_string(),
            "1c5509df07dd8f74c9bfb46d7500b3b89f381c3197ce88bbf5bfae1bf795e919".to_string(),
            "dfbf4654354900cd2580068b8baaf10e3a78ba7fff3e594eeef71ff34435c1f6".to_string(),
        ];
        let proof_index = 0;
        let my_word = "like".to_string();
        let is_correct = check_proof(&proof, &proof_index, &my_word);
        assert_eq!(is_correct, true);
    }
}
