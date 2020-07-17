use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let leaves = get_data();
    println!("{:?}", leaves);
    power_of_2_check(leaves.len());
    let proof_index = 0;
    let first_hashed_leaves = first_hashing(&leaves, &proof_index);
    println!("{:?}", first_hashed_leaves.hashed_leaves);
    println!("{:?}", first_hashed_leaves.sister_hash);
    // let mut nextHash = 
    let mut root = first_hashed_leaves.hashed_leaves;
    let mut proof = Vec::new();
    proof.push(first_hashed_leaves.sister_hash);
    let mut adjacentHash = first_hashed_leaves.sister_hash;
    while root.len() > 1 {
        let return_data = reduce_merkle_branches(root, adjacentHash);
        root = return_data.row;
        adjacentHash = return_data.adjacentHash;
        proof.push(return_data.adjacentHash)
    }
    // TODO deal with leaves borrow checker better
    // TODO put full proof into struct
    let isProved = check_proof(&proof, &proof_index, &leaves[proof_index]);

    println!("proof {:?}, leaf_index {}, my_word {}", &proof, &proof_index, leaves[proof_index]);
    println!("I am root {:?}", root);
    println!("is proved? {:?}", isProved);


}

struct MerkleBranchReturn {
    row: Vec<u64>,
    adjacentHash: u64
}

fn reduce_merkle_branches(nodes: Vec<u64>, adjacentHash: u64) -> MerkleBranchReturn {
    let mut row = Vec::with_capacity((nodes.len() + 1) / 2);
    let mut i = 0;
    while i < nodes.len() {
        // I am safe to assume even branches due to the power of two check but if I wanted to ease restrictions 
        // I would if statment here if none nodes i + 1 and send through i twice (hash left twice)
        row.push(hash_nodes(nodes[i], nodes[i + 1]));
        i += 2;
    }
    let adjacent_hash_index = nodes.iter().position(|&r| r == adjacentHash).unwrap();
    let next_index_level = adjacent_hash_index / 2;
    println!("index positio {}", adjacent_hash_index);
    println!("next_index_level positio {}", next_index_level);
    if row.len() < 2 {
        return MerkleBranchReturn {
            adjacentHash: row[0],
            row
        }
    }
    let sister_hash = if next_index_level % 2 == 0 {row[next_index_level + 1]} else {row[next_index_level - 1]};
    println!("{:?}", row);
    MerkleBranchReturn {
        row, 
        adjacentHash: sister_hash
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
    sister_hash: u64

}

fn first_hashing(leaves: &Vec<String>, index: &usize) -> FirstHashReturn {
    let mut hasher = DefaultHasher::new();
    let mut hashed_leaves = Vec::new();
    hashed_leaves = leaves.iter()
                        .map(|x| {
                            x.hash(&mut hasher); 
                            hasher.finish()
                        })
                                    .collect();
    let sister_hash = if index % 2 == 0 {hashed_leaves[index + 1]} else {hashed_leaves[index - 1]};
    let return_data = FirstHashReturn {
        sister_hash, 
        hashed_leaves
    };
    

    return_data
}

fn check_proof(nodes: &Vec<u64>, index: &usize, word: &String) -> bool { 
    let mut hasher = DefaultHasher::new();
    word.hash(&mut hasher); 
    let hashed_word = hasher.finish();
    let first_level = if index % 2 == 0 {hash_function(hashed_word.wrapping_add(nodes[0]))} else {hash_function(nodes[0].wrapping_add(hashed_word))};
    println!("first_level {:?}", first_level);

    let mut i = 1;
    let mut current_hash = first_level;
    while i  < nodes.len() - 1 {
        current_hash = (reduce_proof(nodes[i], current_hash, index, i));
        println!("current_hash {}", current_hash);
        i += 1; 
    }
    current_hash == nodes[nodes.len() - 1]
}

fn reduce_proof(next_hash: u64, current_hash: u64, leaf_index: &usize, i: usize) -> u64{
    let new_position = (leaf_index / i + 2);
    if new_position % 2 == 0 {hash_function(current_hash.wrapping_add(next_hash))} else {hash_function(next_hash.wrapping_add(current_hash))}
}

fn hash_function(data: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher); 
    hasher.finish()
}

fn power_of_2_check(length: usize) {

 if length == 0 {
    panic!("hey wait no stop");
 }

 let is_power_of_2 = (length & (length - 1)) == 0;
    match is_power_of_2 {
        true => (),
        false => panic!("hey wait no stop")
    }
}
fn get_data() ->  Vec<String> {
    vec!["like".into(), "this".into(), "that".into(), "and".into(), "this".into(), "and".into(), "that".into(), "so".into(), "just".into(), "chill".into(), "till".into(), "the".into(), "next".into(), "episode".into(), "one".into(), "two".into()]
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic(expected = "hey wait no stop")]
    fn it_fails_not_power_of_two() {
        power_of_2_check(15);
    }
    #[test]
    #[should_panic(expected = "hey wait no stop")]
    fn it_fails_if_0() {
        power_of_2_check(0);
    }
    #[test]
    fn it_passes_power_of_two() {
        power_of_2_check(16);
    }
    #[test]
    fn it_hashes_values() {
        let test_data = vec!["like".into(), "this".into()];
        let hashed_test_data = first_hashing(&test_data, &0);
        let expected_result = vec![13469705049872891777, 2396052557377466138];
        assert_eq!(expected_result, hashed_test_data.hashed_leaves);
        assert_eq!(expected_result[1], hashed_test_data.sister_hash);
    }

    #[test]
    fn it_calculates_root() {
        let expected_result = vec![16637296205013643304];
        let first_hashed_leaves = first_hashing(&get_data(), &0);
        let mut root = first_hashed_leaves.hashed_leaves;
        let mut adjacentHash = first_hashed_leaves.sister_hash;
        while root.len() > 1 {
            let return_data = reduce_merkle_branches(root, adjacentHash);
            root = return_data.row;
            adjacentHash = return_data.adjacentHash;
        }
        assert_eq!(root, expected_result)
    }

}