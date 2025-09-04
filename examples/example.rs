use anproto::*;

fn main() {
    let m = "Hello World";
    let h = hash(m);
    let k = gen().unwrap();
    let s = sign(&h, &k).unwrap();
    let o = open(&s).unwrap();
    println!("{}", k);
    println!("{}", h);
    println!("{}", s);
    println!("{}", o);
}
