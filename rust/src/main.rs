fn main() {
    let mut leaves = get_data();
    println!("{:?}", leaves)
}

fn get_data() ->  Vec<String> {
    vec!["like".into(), "this".into(), "that".into(), "and".into(), "this".into(), "and".into(), "that".into(), "so".into(), "just".into(), "chill".into(), "till".into(), "the".into(), "next".into(), "episode".into(), "one".into(), "two".into()]
}