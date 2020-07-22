use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let leaves = get_data();
    let proof_index = 0;
    let first_hashed_leaves = first_hashing(&leaves, &proof_index);
    println!("{:?}", first_hashed_leaves.hashed_leaves);
    println!("{:?}", first_hashed_leaves.sister_hash);
    let mut root = first_hashed_leaves.hashed_leaves.clone();
    let mut proof = Vec::new();
    proof.push(first_hashed_leaves.sister_hash);
    let mut adjacent_hash = first_hashed_leaves.sister_hash;
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

struct MerkleBranchReturn {
    row: Vec<u64>,
    adjacent_hash: u64,
}

fn reduce_merkle_branches(nodes: Vec<u64>, adjacent_hash: u64) -> MerkleBranchReturn {
    let mut row = Vec::with_capacity((nodes.len() + 1) / 2);
    let mut i = 0;
    while i < nodes.len() {
        if nodes.len() - row.len() * 2 != 1 {
            row.push(hash_nodes(nodes[i], nodes[i + 1]));
        } else {
            row.push(hash_nodes(nodes[i], nodes[i]));
        }
        i += 2;
    }
    let adjacent_hash_index = nodes.iter().position(|&r| r == adjacent_hash).unwrap();
    let next_index_level = adjacent_hash_index / 2;
    if row.len() < 2 {
        return MerkleBranchReturn {
            adjacent_hash: row[0],
            row,
        };
    }
    let sister_hash = if next_index_level % 2 == 0 {
        row[next_index_level + 1]
    } else {
        row[next_index_level - 1]
    };
    println!("{:?}", row);
    MerkleBranchReturn {
        row,
        adjacent_hash: sister_hash,
    }
}

fn hash_nodes(left: u64, right: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    let concat = left.wrapping_add(right);
    concat.hash(&mut hasher);
    hasher.finish()
}
struct FirstHashReturn {
    hashed_leaves: Vec<u64>,
    sister_hash: u64,
}
fn first_hashing(leaves: &Vec<String>, index: &usize) -> FirstHashReturn {
    let mut hashed_leaves = Vec::new();
    hashed_leaves = leaves
        .iter()
        .map(|x| {
            let mut hasher = DefaultHasher::new();
            x.hash(&mut hasher);
            hasher.finish()
        })
        .collect();
    let sister_hash = if index % 2 == 0 {
        hashed_leaves[index + 1]
    } else {
        hashed_leaves[index - 1]
    };
    let return_data = FirstHashReturn {
        sister_hash,
        hashed_leaves,
    };

    return_data
}

fn check_proof(nodes: &Vec<u64>, index: &usize, word: &String) -> bool {
    let mut hasher = DefaultHasher::new();
    word.hash(&mut hasher);
    let hashed_word = hasher.finish();

    // this does not matter with u64 as adding two numbers together yields same result, however will work when/if switch hash functions
    let first_level = if index % 2 == 0 {
        hash_function(hashed_word.wrapping_add(nodes[0]))
    } else {
        hash_function(nodes[0].wrapping_add(hashed_word))
    };

    let mut i = 1;
    let mut current_hash = first_level;
    while i < nodes.len() - 1 {
        current_hash = reduce_proof(nodes[i], current_hash, index, i);
        // println!("current_hash {}", current_hash);
        i += 1;
    }
    current_hash == nodes[nodes.len() - 1]
}

fn reduce_proof(next_hash: u64, current_hash: u64, leaf_index: &usize, i: usize) -> u64 {
    let new_position = leaf_index / i + 2;
    // this does not matter with u64 as adding two numbers together yields same result, however will work when/if switch hash functions
    if new_position % 2 == 0 {
        hash_function(current_hash.wrapping_add(next_hash))
    } else {
        hash_function(next_hash.wrapping_add(current_hash))
    }
}

fn hash_function(data: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

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
        let expected_result = vec![13469705049872891777, 13421249885991295001];
        assert_eq!(expected_result, hashed_test_data.hashed_leaves);
        assert_eq!(expected_result[1], hashed_test_data.sister_hash);
    }

    #[test]
    fn it_calculates_root() {
        let expected_result = vec![8215901497871711904];
        let first_hashed_leaves = first_hashing(&get_data(), &0);
        let mut root = first_hashed_leaves.hashed_leaves;
        let mut adjacent_hash = first_hashed_leaves.sister_hash;
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
            13421249885991295001,
            9426091635773224930,
            16995962029822891419,
            14703833076160150437,
            8215901497871711904,
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
            13421249885991295001,
            9426091635773224930,
            16995962029822891419,
            14703833076160150437,
            8215901497871711904,
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
            13421249885991295101,
            9426091635773224930,
            16995962029822891419,
            14703833076160150437,
            8215901497871711904,
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
        let expected_result = vec![17278646585610960701];
        let first_hashed_leaves = first_hashing(&data, &0);
        let mut root = first_hashed_leaves.hashed_leaves;
        let mut adjacent_hash = first_hashed_leaves.sister_hash;
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
            13421249885991295001,
            9426091635773224930,
            16995962029822891419,
            2239737509107465412,
            17278646585610960701,
        ]
        .to_vec();
        let proof_index = 0;
        let my_word = "like".to_string();
        let is_correct = check_proof(&proof, &proof_index, &my_word);
        assert_eq!(is_correct, true);
    }
}
