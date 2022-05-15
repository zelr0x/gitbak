use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop)]
pub enum Auth {
    BearerToken(Box<String>),
}
