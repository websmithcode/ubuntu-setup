pub fn vec_of_strings(items: Vec<&str>) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}
