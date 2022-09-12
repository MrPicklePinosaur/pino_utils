//! Comparison related utilities

/// Compares only the enum variant and not the contained value
pub fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
