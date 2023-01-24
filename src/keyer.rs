pub fn domain_key(id: &String) -> String {
    format!("domain:{}", id)
}

pub fn domain_owner_key(id: &String, owner: &String) -> String {
    format!("domain_owner:{}{}", owner, id)
}
