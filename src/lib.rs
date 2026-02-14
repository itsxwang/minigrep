

pub fn search_sensitive<'a>(content: &'a str, query: &str) -> Option<&'a str> {
    if content.contains(query) {
        Some(content)
    } else {
        None
    }
}

pub fn search_case_insensitive<'a>(content: &'a str, query: &str) -> Option<&'a str> {
    if content.to_lowercase().contains(&query.to_lowercase()) {
        Some(content)
    } else {
        None
    }
}

pub fn search_case_smart<'a>(content: &'a str, query: &str) -> Option<&'a str> {
    //  iterate over each char of query and see is any char uppercase, if yes don't search case insensitive, use normal search
    for c in query.chars() {
        if c.is_uppercase() {
            return search_sensitive(content, query);
        }
    }
    search_case_insensitive(content, query)
}
