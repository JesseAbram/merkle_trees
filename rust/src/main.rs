use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let leaves = get_data();
    println!("{:?}", leaves);
    power_of_2_check(leaves.len());
    let first_hashed_leaves = first_hashing(leaves);
    println!("{:?}", first_hashed_leaves);

}

fn first_hashing(leaves: Vec<String>) -> Vec<u64>{
    let mut hasher = DefaultHasher::new();
    let hashed_leaves = leaves.iter()
                        .map(|x| {
                            x.hash(&mut hasher); 
                            hasher.finish()
                        })
                                    .collect();
    hashed_leaves
}

fn power_of_2_check(length: usize) {
 let is_power_of_2 = (length & (length - 1)) == 0;
    if !is_power_of_2 {
        panic!("hey wait no stop")
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
    fn it_passes_power_of_two() {
        power_of_2_check(16);
    }

    fn it_hashes_values() {
        let test_data = vec!["like".into(), "this".into()];
        let hashed_test_data = first_hashing(test_data);
        let expected_result = vec![13469705049872891777, 2396052557377466138];
        assert_eq!(expected_result, hashed_test_data);
    }
}