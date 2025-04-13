// Fix the code so that changing the value of the variable vegetable from 🥦 to 🥕 works. Modify only the line that defines the variable (line 2).

// The fixed code should print out
// 🥦
// 🥕

fn main() {
    let mut vegetable = "🥦";
    println!("{vegetable}");
    vegetable = "🥕";
    println!("{vegetable}");
}