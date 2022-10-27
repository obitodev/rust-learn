///////////////////////////////////////////
// BAI 1
// Yêu cầu :
// + Sửa code liên quan tới vấn đề generic type (thay đổi ở định nghĩa struct)
///////////////////////////////////////////

#[derive(Debug)]
struct Point<T, X> {
    x: T,
    y: X,
}

fn main() {
    // DON'T modify this code.
    let p = Point{x: 5, y : "hello".to_string()};

    println!("Success!");
    println!("{:?}", p);
}