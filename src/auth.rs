pub enum Auth<'a> {
    BearerToken(&'a str),
}
