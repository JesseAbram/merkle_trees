use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let leaves = get_data();
    let proof_index = 0;
    let first_hashed_leaves = first_hashing(&leaves, &proof_index);
    let mut root = first_hashed_leaves.hashed_leaves.clone();
    let mut proof = Vec::new();
    proof.push(first_hashed_leaves.adjacent_hash);
    let mut adjacent_hash = first_hashed_leaves.adjacent_hash;
    while root.len() > 1 {
        let return_data = reduce_merkle_branches(root, adjacent_hash);
        root = return_data.row;
        adjacent_hash = return_data.adjacent_hash;
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
    row: Vec<u64>,
    adjacent_hash: u64,
}
/// takes an array and hashes the adjacent hashes together
/// returns the adjacent hash for the proof
/// handles uneven tree size by hashing the last node twice if needed
fn reduce_merkle_branches(nodes: Vec<u64>, current_adjacent_hash: u64) -> MerkleBranchReturn {
    let mut row = Vec::with_capacity((nodes.len() + 1) / 2);
    let mut i = 0;
    // loops through the nodes and hashes adjacent nodes returning the next layer of the tree
    while i < nodes.len() {
        // handles uneven tree by adding the same node twice
        if nodes.len() - row.len() * 2 != 1 {
            row.push(hash_nodes(nodes[i], nodes[i + 1]));
        } else {
            row.push(hash_nodes(nodes[i], nodes[i]));
        }
        i += 2;
    }
    // for the proof, finds where the current adjacent hash is
    let adjacent_hash_index = nodes
        .iter()
        .position(|&r| r == current_adjacent_hash)
        .unwrap();
    // divides it by two to represent the position in the next layer of the tree
    let next_index_level = adjacent_hash_index / 2;
    // if this is the last level exists here with the root
    if row.len() < 2 {
        return MerkleBranchReturn {
            adjacent_hash: row[0],
            row,
        };
    }
    // if the index position is even get the node to the right, odd get the node to the left
    let adjacent_hash = if next_index_level % 2 == 0 {
        row[next_index_level + 1]
    } else {
        row[next_index_level - 1]
    };
    MerkleBranchReturn { row, adjacent_hash }
}
/// hashes a left and right node together returns hash
fn hash_nodes(left: u64, right: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    let concat = left.wrapping_add(right);
    concat.hash(&mut hasher);
    hasher.finish()
}
/// return type for first_hashing function
struct FirstHashReturn {
    hashed_leaves: Vec<u64>,
    adjacent_hash: u64,
}
/// takes a data set of vec of strings and hashes them
/// returns the hashes of all the input data and the adjacent hash of the leaf to prove
fn first_hashing(leaves: &Vec<String>, index: &usize) -> FirstHashReturn {
    let mut hashed_leaves = Vec::new();
    let mut i = 0;
    // hashes all the leaves, appends an index to force unique hash (almost like a nonce)
    hashed_leaves = leaves
        .iter()
        .map(|x| {
            let unique_word = x.clone() + (&i.to_string());
            let mut hasher = DefaultHasher::new();
            unique_word.hash(&mut hasher);
            i += 1;
            hasher.finish()
        })
        .collect();
    // find adjacent hash, if even it is the right if odd it is the left
    let adjacent_hash = if index % 2 == 0 {
        hashed_leaves[index + 1]
    } else {
        hashed_leaves[index - 1]
    };
    FirstHashReturn {
        adjacent_hash,
        hashed_leaves,
    }
}
/// takes in proof, recreates the tree based on the proof
/// returns true if the root matches
fn check_proof(nodes: &Vec<u64>, index: &usize, word: &String) -> bool {
    // hash the piece of data to be checked
    let mut hasher = DefaultHasher::new();
    let unique_word = word.clone() + &index.to_string();
    unique_word.hash(&mut hasher);
    let hashed_word = hasher.finish();

    // this does not matter with u64 as adding two numbers together yields same result, however will work when/if switch hash functions
    // hashes the first level of the tree
    let first_level = if index % 2 == 0 {
        hash_nodes(hashed_word, nodes[0])
    } else {
        hash_nodes(nodes[0], hashed_word)
    };

    let mut i = 1;
    let mut current_hash = first_level;
    let mut new_index = *index;
    // hashes the next levels to get the root
    while i < nodes.len() - 1 {
        let return_data = reduce_proof(nodes[i], current_hash, new_index);
        current_hash = return_data.hashed_leaves;
        new_index = return_data.new_index_position;

        i += 1;
    }
    // if the roots match return true
    current_hash == nodes[nodes.len() - 1]
}

struct ReduceProofReturn {
    hashed_leaves: u64,
    new_index_position: usize,
}
/// takes two nodes of the tree and hashes them to create the next level of the tree
fn reduce_proof(next_hash: u64, current_hash: u64, leaf_index: usize) -> ReduceProofReturn {
    // the next position in the tree is the current position divided by two and rounded down but u64 handles
    let new_index_position = leaf_index / 2;
    let hashed_leaves;
    // this does not matter with u64 as adding two numbers together yields same result, however will work when/if switch hash functions
    if new_index_position % 2 == 0 {
        hashed_leaves = hash_nodes(current_hash, next_hash);
    } else {
        hashed_leaves = hash_nodes(current_hash, next_hash);
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
        "two".into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_hashes_values() {
        let test_data = vec!["like".into(), "this".into()];
        let hashed_test_data = first_hashing(&test_data, &0);
        let expected_result = vec![14766127051168428186, 2096896405943510157];
        assert_eq!(expected_result, hashed_test_data.hashed_leaves);
        assert_eq!(expected_result[1], hashed_test_data.adjacent_hash);
    }

    #[test]
    fn it_calculates_root() {
        let expected_result = vec![8399560962055331074];
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
        let expected_result = 11396221907974800208;
        let left = 13469705049872891777;
        let right = 13421249885991295001;
        let hashed_node = hash_nodes(left, right);
        assert_eq!(hashed_node, expected_result);
    }

    #[test]
    fn it_calculates_correct_proof_to_true() {
        let proof = [
            2096896405943510157,
            4853846757327170105,
            12915908306703903221,
            15635660685162264787,
            8399560962055331074,
        ]
        .to_vec();
        let proof_index = 0;
        let my_word = "like".to_string();
        let is_correct = check_proof(&proof, &proof_index, &my_word);
        assert_eq!(is_correct, true);
    }
    #[test]
    fn it_calculates_incorrect_proof_to_false_with_wrong_word() {
        let proof = [
            2096896405943510157,
            4853846757327170105,
            12915908306703903221,
            15635660685162264787,
            8399560962055331074,
        ]
        .to_vec();
        let proof_index = 0;
        let my_word = "this".to_string();
        let is_correct = check_proof(&proof, &proof_index, &my_word);
        assert_eq!(is_correct, false);
    }
    #[test]
    fn it_calculates_incorrect_proof_to_false_with_wrong_proof() {
        let proof = [
            2096896405943510158,
            4853846757327170105,
            12915908306703903221,
            15635660685162264787,
            8399560962055331074,
        ]
        .to_vec();
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
        let expected_result = vec![14764477793080918021];
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
        let proof = [
            2096896405943510157,
            4853846757327170105,
            12915908306703903221,
            5642424714509190392,
            14764477793080918021,
        ]
        .to_vec();
        let proof_index = 0;
        let my_word = "like".to_string();
        let is_correct = check_proof(&proof, &proof_index, &my_word);
        assert_eq!(is_correct, true);
    }
}
