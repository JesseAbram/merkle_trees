use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let leaves = get_data();
    println!("{:?}", leaves);
    power_of_2_check(leaves.len());
    let first_hashed_leaves = first_hashing(leaves, 0);
    println!("{:?}", first_hashed_leaves.hashed_leaves);
    println!("{:?}", first_hashed_leaves.sister_hash);

    // let mut root = [];
    let mut root = Vec::new();
    root = first_hashed_leaves.hashed_leaves;
    while root.len() > 1 {
        root = reduce_merkle_branches(root);
    }
    println!("I am root {:?}", root);

}

fn reduce_merkle_branches(nodes: Vec<u64>) -> Vec<u64> {
    let mut row = Vec::with_capacity((nodes.len() + 1) / 2);
    let mut i = 0;
    while i < nodes.len() {
        // I am safe to assume even branches due to the power of two check but if I wanted to ease restrictions 
        // I would if statment here if none nodes i + 1 and send through i twice (hash left twice)
        row.push(hash_nodes(nodes[i], nodes[i + 1]));
        i += 2;
    }
    println!("{:?}", row);

    row
}

fn hash_nodes(left: u64, right: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    let concat = left.wrapping_add(right);
    concat.hash(&mut hasher);
    hasher.finish()

}
struct first_hash_return {
    hashed_leaves: Vec<u64>,
    sister_hash: u64

}

fn first_hashing(leaves: Vec<String>, index: usize) -> first_hash_return {
    let mut hasher = DefaultHasher::new();
    let mut hashed_leaves = Vec::new();
    hashed_leaves = leaves.iter()
                        .map(|x| {
                            x.hash(&mut hasher); 
                            hasher.finish()
                        })
                                    .collect();
    let sister_hash = if index % 2 == 0 {hashed_leaves[index + 1]} else {hashed_leaves[index - 1]};
    let return_data = first_hash_return {
        sister_hash, 
        hashed_leaves
    };
    

    return_data
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
        let hashed_test_data = first_hashing(test_data, 0);
        let expected_result = vec![13469705049872891777, 2396052557377466138];
        assert_eq!(expected_result, hashed_test_data.hashed_leaves);
    }

    #[test]
    fn it_calculates_root() {
        let expected_result = vec![16637296205013643304];
        let first_hashed_leaves = first_hashing(get_data(), 0);
        let mut root = Vec::new();
        root = first_hashed_leaves.hashed_leaves;

        while root.len() > 1 {
            root = reduce_merkle_branches(root);
        }
        assert_eq!(root, expected_result)
    }

}