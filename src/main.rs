use std::io;
use std::time::Instant;

fn is_pallindrome (x: i32) -> bool {
    if x < 0 {return false;}

    let original = x;
    let mut temp = original.clone();
    let mut reversed = 0;

    while temp != 0 {
        reversed = reversed * 10 + temp % 10;
        temp /= 10;
    }

    original == reversed
}

fn main() {
    let mut n = String::new();

    println!("Input number: ");
    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n = match n.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Bad input");
            return;
        }
    };

    let now = Instant::now();

    if is_pallindrome(n) {
        println!("{n}");
        println!("Time taken: {} milliseconds", now.elapsed().as_millis());
        return;
    }

    let mut palindromes: Vec<i32> = Vec::new();

    for i in 1..n {
        if is_pallindrome(i) {
            palindromes.push(i);
        }
    }

    for &x in &palindromes {
        let y = n - x;
        if palindromes.contains(&y) {
            println!("{x} {y}");
            println!("Time taken: {} milliseconds", now.elapsed().as_millis());
            return;
        }
    }

    for &x in &palindromes {
        for &y in &palindromes {
            let z = n - x - y;
            if z < 0 {return;}
            if palindromes.contains(&z) {
                println!("{x} {y} {z}");
                println!("Time taken: {} milliseconds", now.elapsed().as_millis());
                return;
            }
        }
    }
}