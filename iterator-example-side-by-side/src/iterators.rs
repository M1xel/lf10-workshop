// Iterator style - elegant, safe, and expressive!

pub fn demo_iterators() {
    println!("⚡ ITERATOR WAY - Functional Style\n");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let words = vec!["hello", "world", "rust", "is", "awesome", "programming"];
    let ages = vec![25, 30, 17, 42, 19, 33, 16, 28];

    // 1. FIND FIRST EVEN NUMBER > 5
    println!("1. Find first even number > 5:");
    let result = numbers.iter().find(|&&x| x % 2 == 0 && x > 5);
    println!("Result: {:?}", result); // Some(6)

    // 2. SUM OF SQUARES OF EVEN NUMBERS
    println!("\n2. Sum of squares of even numbers:");
    let sum: i32 = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();
    println!("Sum: {}", sum); // 220

    // 3. COLLECT WORDS LONGER THAN 4 CHARACTERS
    println!("\n3. Words longer than 4 characters:");
    let long_words: Vec<&str> = words.iter()
        .filter(|&word| word.len() > 4)
        .copied()
        .collect();
    println!("Long words: {:?}", long_words); // ["hello", "world", "awesome", "programming"]

    // 4. COUNT ADULTS (age >= 18)
    println!("\n4. Count adults (age >= 18):");
    let adult_count = ages.iter().filter(|&&age| age >= 18).count();
    println!("Adults: {}", adult_count); // 6

    // 5. TRANSFORM AND FILTER - Double numbers, keep > 10
    println!("\n5. Double numbers, keep those > 10:");
    let doubled_filtered: Vec<i32> = numbers.iter()
        .map(|&x| x * 2)
        .filter(|&x| x > 10)
        .collect();
    println!("Result: {:?}", doubled_filtered); // [12, 14, 16, 18, 20]

    // 6. COMBINE TWO LISTS - Pair ages with status
    println!("\n6. Pair ages with adult status:");
    let age_status: Vec<(i32, &str)> = ages.iter()
        .map(|&age| (age, if age >= 18 { "Adult" } else { "Minor" }))
        .collect();
    println!("Age-Status pairs: {:?}", age_status);

    // 7. FIND MAXIMUM VALUE
    println!("\n7. Find maximum value:");
    let max_val = numbers.iter().max();
    println!("Maximum: {:?}", max_val); // Some(10)

    // 8. CHECK IF ALL NUMBERS ARE POSITIVE
    println!("\n8. Check if all numbers are positive:");
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("All positive: {}", all_positive); // true

    // BONUS: ADVANCED CHAINING
    println!("\n🚀 BONUS - Complex chaining:");
    let complex_result: Vec<String> = words.iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)  // Even indices only
        .map(|(_, &word)| word.to_uppercase())
        .filter(|word| word.len() > 4)
        .collect();
    println!("Even-indexed long words (uppercase): {:?}", complex_result);

    println!("\n✅ BENEFITS OF ITERATOR APPROACH:");
    println!("  • Concise and expressive");
    println!("  • No manual index management");
    println!("  • Immutable by default");
    println!("  • Easy to chain operations");
    println!("  • Intent is crystal clear");
    println!("  • Zero-cost abstractions!");
    println!("  • Compiler prevents many errors");
}