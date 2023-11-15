fn main() {
    let mut x = 5;
    let y = &mut x;
    *y=1;

    let vec = vec![3, 4, 5];
    let y = vec;
    // println!("{}",vec.len()); // error[E0382]: borrow of moved value: `vec`
}
