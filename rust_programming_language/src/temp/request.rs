use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
pub struct QueryParser<'a> {
    query: &'a str,
    params: HashMap<&'a str, &'a str>,
}
#[allow(dead_code)]
impl<'a> QueryParser<'a> {
    pub fn from_string(query: &'a str) -> Self {
        let params: HashMap<&'a str, &'a str> = query
            .split("&")
            .map(|kv| {
                let mut parts = kv.split("=");
                (parts.next().unwrap(), parts.next().unwrap())
            })
            .collect();
        QueryParser { query, params }
    }
}
