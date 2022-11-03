// Bài 1
// Sửa code để compile thành công liên quan tới vấn đề lifetime
/////////////////////////////////////

// use std::io;
// fn main() {
//     let mut input: Vec<String>;
//     loop {
//         let mut input_text = String::new();
//         println!("Type instruction in the format Add <name> to <department>:");
//         io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
//         let trimmed_text: String = input_text.trim().to_string();
//         input = trimmed_text.split(" ").map(|x| x.to_string()).collect();
//         if input[0] == "Add" && input[2] == "to" {
//             break;
//         } else {
//             println!("Invalid format.");
//         }
//     }
//     println!("{:?}", input);
// }

/////////////////////////////////////
// Bài 2
// Implement trait
// Thực hiên cargo test
/////////////////////////////////////


// trait AppendBar {
//     fn append_bar(self) -> Self;
// }

// impl AppendBar for String {
//     //Add your code here
//     fn append_bar(mut self) -> Self {
//         self.push_str("Bar");
//         self
//     }
// }

// fn main() {
//     let s = String::from("Foo");
//     let s = s.append_bar();
//     println!("s: {}", s);
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn is_foo_bar() {
//         assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
//     }

//     #[test]
//     fn is_bar_bar() {
//         assert_eq!(
//             String::from("").append_bar().append_bar(),
//             String::from("BarBar")
//         );
//     }
// }