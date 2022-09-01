//! Specialized collections

/// An augmneted vector that contains a 'needle' that points to an index in the vector
///
/// Can be used to implement a list where items may be inserted and removed at any time
pub struct SelectionVec<T> {
    contents: Vec<T>,
    needle: usize 
}

impl<T> SelectionVec<T> {

}
