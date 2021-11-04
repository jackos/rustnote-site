fn replace_with_84(s: &mut Box<i32>) {
    // this is not okay, as *s would be empty:
    let was = *s;
    // but this is:
    let was = std::mem::take(s);
    // so is this:
    *s = was;
    // we can exchange values behind &mut:
    let mut r = Box::new(84);
    std::mem::swap(s, &mut r);
}

fn main() {
    let mut s = Box::new(42);
    replace_with_84(&mut s);
}
