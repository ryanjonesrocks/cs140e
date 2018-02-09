// FIXME: Make me compile! Diff budget: 2 lines.

// Do not modify the inner type &'a T.
// Needs a declared lifetime
struct RefWrapper<'a, T: 'a>(&'a T);

// Do not modify the inner type &'b RefWrapper<'a, T>.
// Needs a declared lifetime
struct RefWrapperWrapper<'a: 'b, 'b, T: 'a >(&'b RefWrapper<'a, T>);

pub fn main() { }
