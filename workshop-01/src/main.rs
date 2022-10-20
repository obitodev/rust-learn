fn main() {
    println!("Hello, world!");
    // Ex01
    // let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    // let sub_arr = [6, 8, 10];

    // let org_str: String = org_arr
    //     .into_iter()
    //     .map(|i| i.to_string())
    //     .collect::<String>();
    // let sub_str: String = sub_arr
    //     .into_iter()
    //     .map(|i| i.to_string())
    //     .collect::<String>();

    // if org_str.contains(&sub_str) {
    //     println!("sub_str là mảng con")
    // } else {
    //     println!("sub_str khong là mảng con")
    // }

    // Ex02
    let mut input = String::from("adbcdaDd").to_ascii_lowercase();
    let mut line = String::new();
    println!("Enter 1 char :");
    std::io::stdin().read_line(&mut line).unwrap();
    let input_char = line.chars().nth(0).unwrap();

    let mut count = 0;
    for c in input.chars() {
        if input_char == c {
            count += 1;
        }
    }

    input = input.replace(input_char, "");

    println!("{}, {}", count, input)
}
