fn main()
{
    let mut v = Vec::new();
    v.push("Hello,");
    let x = &v[0];
    //breaks invariant
    v.push(" world!"); // statically disallowed
    println!("{}", x);
}
