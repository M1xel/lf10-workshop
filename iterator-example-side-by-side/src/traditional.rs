// Traditional imperative style - lots of manual work!

pub fn demo_traditional() {
    println!("🔧 TRADITIONAL WAY - Imperative Style\n");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let words = vec!["hello", "world", "rust", "is", "awesome", "programming"];
    let ages = vec![25, 30, 17, 42, 19, 33, 16, 28];

    // 1. FIND FIRST EVEN NUMBER > 5
    println!("1. Find first even number > 5:");
    let mut result = None;
    for i in 0..numbers.len() {
        if numbers[i] % 2 == 0 && numbers[i] > 5 {
            result = Some(numbers[i]);
            break;
        }
    }
    println!("Result: {:?}", result); // Some(6)

    // 2. SUM OF SQUARES OF EVEN NUMBERS
    println!("\n2. Sum of squares of even numbers:");
    let mut sum = 0;
    for i in 0..numbers.len() {
        if numbers[i] % 2 == 0 {
            sum += numbers[i] * numbers[i];
        }
    }
    println!("Sum: {}", sum); // 220 (2²+4²+6²+8²+10² = 4+16+36+64+100)

    // 3. COLLECT WORDS LONGER THAN 4 CHARACTERS
    println!("\n3. Words longer than 4 characters:");
    let mut long_words = Vec::new();
    for i in 0..words.len() {
        if words[i].len() > 4 {
            long_words.push(words[i]);
        }
    }
    println!("Long words: {:?}", long_words); // ["hello", "world", "awesome", "programming"]

    // 4. COUNT ADULTS (age >= 18)
    println!("\n4. Count adults (age >= 18):");
    let mut adult_count = 0;
    for i in 0..ages.len() {
        if ages[i] >= 18 {
            adult_count += 1;
        }
    }
    println!("Adults: {}", adult_count); // 6

    // 5. TRANSFORM AND FILTER - Double numbers, keep > 10
    println!("\n5. Double numbers, keep those > 10:");
    let mut doubled_filtered = Vec::new();
    for i in 0..numbers.len() {
        let doubled = numbers[i] * 2;
        if doubled > 10 {
            doubled_filtered.push(doubled);
        }
    }
    println!("Result: {:?}", doubled_filtered); // [12, 14, 16, 18, 20]

    // 6. COMBINE TWO LISTS - Pair ages with status
    println!("\n6. Pair ages with adult status:");
    let mut age_status = Vec::new();
    for i in 0..ages.len() {
        let status = if ages[i] >= 18 { "Adult" } else { "Minor" };
        age_status.push((ages[i], status));
    }
    println!("Age-Status pairs: {:?}", age_status);

    // 7. FIND MAXIMUM VALUE
    println!("\n7. Find maximum value:");
    let mut max_val = numbers[0];
    for i in 1..numbers.len() {
        if numbers[i] > max_val {
            max_val = numbers[i];
        }
    }
    println!("Maximum: {}", max_val); // 10

    // 8. CHECK IF ALL NUMBERS ARE POSITIVE
    println!("\n8. Check if all numbers are positive:");
    let mut all_positive = true;
    for i in 0..numbers.len() {
        if numbers[i] <= 0 {
            all_positive = false;
            break;
        }
    }
    println!("All positive: {}", all_positive); // true

    println!("\n❌ PROBLEMS WITH TRADITIONAL APPROACH:");
    println!("  • Verbose and repetitive");
    println!("  • Manual index management (error-prone)");
    println!("  • Mutable variables everywhere");
    println!("  • Hard to chain operations");
    println!("  • Intent buried in implementation details");
    println!("  • Risk of off-by-one errors");
}