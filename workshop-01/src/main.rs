fn main() {
    println!("Hello, world!");
    // Ex01
    let org_arr = [1,2,3,5,6,8,10,11];
    let sub_arr = [6,8,10]; 

    let org_str: String = org_arr.into_iter().map(|i| i.to_string()).collect::<String>();
    let sub_str: String = sub_arr.into_iter().map(|i| i.to_string()).collect::<String>();

    if org_str.contains(&sub_str) {
        println!("sub_str là mảng con")
    }else{
        println!("sub_str khong là mảng con")
    }

    // Ex02
}
