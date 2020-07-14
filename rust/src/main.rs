fn main() {
    let mut leaves = get_data();
    power_of_2_check(leaves.len());
    println!("{:?}", leaves)
}

fn power_of_2_check(length: usize) {
 let is_power_of_2 = (length & (length - 1)) == 0;
    if (!is_power_of_2) {
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
}