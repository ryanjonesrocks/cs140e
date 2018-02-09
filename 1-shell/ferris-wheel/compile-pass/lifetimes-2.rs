// FIXME: Make me compile! Diff budget: 3 lines.

// Do not modify the inner type &'a T.
// &'a needs a declared lifetime
struct RefWrapper<'a, T: 'a>(&'a T);

//RefWrapper needs at least 1 type argument
impl<'a, T> RefWrapper<'a, T> {
    
    //&T needs to be found in scope
    fn inner(&self) -> &'a T {
        self.0
    }
}

// Do not modify this function.
pub fn main() {
    let x = 1;
    let mut r = &x;
    r = RefWrapper(r).inner();
}
