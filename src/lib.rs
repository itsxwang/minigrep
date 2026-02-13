pub fn search<'a>(s: &'a str, query: &'a str) -> Option<&'a str> {
    if s.contains(query) { Some(s) } else { None }
}
