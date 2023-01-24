pub fn domain_key(id: &String, owner: &String) -> String {
    format!("domain:{}{}", id, owner)
}
