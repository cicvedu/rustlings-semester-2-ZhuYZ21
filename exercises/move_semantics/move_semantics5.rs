fn main() {
    let mut x = 100;
    let y;
    {
        y = &mut x;
        *y += 100;
    }
    let z;
    {
        z = &mut x;
        *z += 1000;
    }
    assert_eq!(x, 1200);
}
