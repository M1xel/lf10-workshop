fn main() {
    println!("🦀 Rust Iterator Workshop - Simple Examples\n");

    // 1. COLLECT - Turn iterator back into a collection
    println!("1. COLLECT - Basic collection");
    let numbers: Vec<i32> = (1..6).collect(); // Output: [1, 2, 3, 4, 5]
    println!("Range (1..6) collected: {:?}\n", numbers);

    // 2. MAP - Transform each element
    println!("2. MAP - Transform each element");
    let doubled: Vec<i32> = (1..6).map(|x| x * 2).collect(); // Output: [2, 4, 6, 8, 10]
    println!("Numbers doubled: {:?}\n", doubled);

    // 3. FILTER - Keep only elements that match
    println!("3. FILTER - Keep matching elements");
    let evens: Vec<i32> = (1..11).filter(|&x| x % 2 == 0).collect(); // Output: [2, 4, 6, 8, 10]
    println!("Even numbers 1-10: {:?}\n", evens);

    // 4. ENUMERATE - Add an index to each element
    println!("4. ENUMERATE - Add position index");
    let words = vec!["apple", "banana", "cherry"];
    let indexed: Vec<_> = words.iter().enumerate().collect(); // Output: [(0, "apple"), (1, "banana"), (2, "cherry")]
    println!("Words with index: {:?}\n", indexed);

    // 5. ZIP - Combine two iterators
    println!("5. ZIP - Combine two collections");
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![25, 30, 35];
    let pairs: Vec<_> = names.iter().zip(ages.iter()).collect(); // Output: [("Alice", 25), ("Bob", 30), ("Charlie", 35)]
    println!("Name-Age pairs: {:?}\n", pairs);

    // 6. TAKE - Get first N elements
    println!("6. TAKE - Get first N elements");
    let first_three: Vec<i32> = (1..100).take(3).collect(); // Output: [1, 2, 3]
    println!("First 3 from 1-100: {:?}\n", first_three);

    // 7. SKIP - Skip first N elements
    println!("7. SKIP - Skip first N elements");
    let after_skip: Vec<i32> = (1..10).skip(5).collect(); // Output: [6, 7, 8, 9]
    println!("Skip first 5 from 1-10: {:?}\n", after_skip);

    // 8. FOR_EACH - Do something with each element (no collect needed!)
    println!("8. FOR_EACH - Print each element");
    print!("Printing 1-5: ");
    (1..6).for_each(|x| print!("{} ", x)); // Output: 1 2 3 4 5
    println!("\n");

    // 9. FIND - Get the first matching element
    println!("9. FIND - First matching element");
    let first_big = (1..10).find(|&x| x > 7); // Output: Some(8)
    println!("First number > 7: {:?}\n", first_big);

    // 10. COUNT - How many elements
    println!("10. COUNT - Count elements");
    let count = (1..100).filter(|&x| x % 10 == 0).count(); // Output: 9
    println!("Numbers 1-100 divisible by 10: {}\n", count);

    // 11. SUM - Add all elements together
    println!("11. SUM - Add all elements");
    let total: i32 = (1..6).sum(); // Output: 15
    println!("Sum of 1-5: {}\n", total);

    // 12. MAX/MIN - Find biggest/smallest
    println!("12. MAX/MIN - Find extremes");
    let numbers = vec![5, 2, 8, 1, 9, 3];
    let biggest = numbers.iter().max(); // Output: Some(9)
    let smallest = numbers.iter().min(); // Output: Some(1)
    println!("In {:?}: max={:?}, min={:?}\n", numbers, biggest, smallest);

    // 13. CHAIN - Connect two iterators
    println!("13. CHAIN - Connect iterators");
    let first = vec![1, 2, 3];
    let second = vec![4, 5, 6];
    let combined: Vec<_> = first.iter().chain(second.iter()).collect(); // Output: [1, 2, 3, 4, 5, 6]
    println!("Chained: {:?}\n", combined);

    // 14. REVERSE - Backwards iteration
    println!("14. REVERSE - Go backwards");
    let backwards: Vec<i32> = (1..6).rev().collect(); // Output: [5, 4, 3, 2, 1]
    println!("5 to 1: {:?}\n", backwards);

    // 15. COMBINING METHODS
    println!("15. COMBINING - Iterator power!");
    let result: Vec<_> = (1..20)
        .filter(|&x| x % 2 == 0)  // Keep evens
        .map(|x| x * x)           // Square them
        .take(3)                  // Take the first 3
        .collect(); // Output: [4, 16, 36]
    println!("Even numbers 1-20, squared, first 3: {:?}", result);
}
