pub fn vec_to_string<T>(vec: &Vec<T>) {
    vec.into_iter().map(|c| c.to_string()).collect::<Vec<String>>()
}
