# 🦀 Rust Iterator Workshop - Student Tasks

**Total Time: ~80 minutes**

## 📋 Workshop Overview

| Section | Topic | Duration | Format |
|---------|-------|----------|---------|
| **Act 1** | Iterator Theory & Examples | 25 min | Presentation |
| **Act 2** | Hands-on: Using Iterators | 35 min | Coding Tasks |
| **Act 3** | Custom Iterator Implementation | 20 min | Guided Coding |

---

## 🎯 Learning Objectives

By the end of this workshop, you will:
- ✅ Understand what iterators are and why they're powerful
- ✅ Use common iterator methods (`map`, `filter`, `collect`, etc.)
- ✅ Chain iterator operations for complex data processing
- ✅ Implement your own custom iterator from scratch
- ✅ Recognize when to use iterators vs traditional loops

---

## 🌐 Online Environment Setup

**Recommended Online Compiler:** [play.rust-lang.org](https://play.rust-lang.org)

**Alternative Options:**
- [replit.com](https://replit.com) (supports multiple languages)
- [Compiler Explorer](https://godbolt.org) (great for seeing assembly output)

---

# 📚 ACT 1: Theory & Examples (25 minutes)

*[Instructor-led presentation with live coding demos]*

### Topics Covered:
1. **What are Iterators?** - The pattern behind data processing
2. **Simple Examples** - Basic iterator methods in action  
3. **Side-by-Side Comparison** - Traditional vs Iterator approaches
4. **Advanced Examples** - Custom iterator implementations

---

# 💻 ACT 2: Hands-On Iterator Usage (35 minutes)

## 🚀 Getting Started

Copy this starter code into your online compiler:

```rust
fn main() {
    // Test data for exercises
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let words = vec!["apple", "banana", "cherry", "date", "elderberry", "fig"];
    let ages = vec![25, 17, 30, 16, 42, 19, 33, 15, 28, 21];
    let prices = vec![10.99, 5.50, 23.00, 8.75, 15.25, 99.99, 2.49];
    
    println!("🦀 Iterator Exercises - Let's Code!\n");
    
    // Your solutions go here!
}
```

---

## 📝 Exercise 1: Basic Iterator Operations (8 minutes)

### Task 1.1: Double All Numbers ⭐
**Goal:** Transform each number by multiplying it by 2

**Expected Output:** `[2, 4, 6, 8, 10, 12, 14, 16, 18, 20]`

```rust
// TODO: Use .map() and .collect() to double all numbers
let doubled = numbers.iter() /* YOUR CODE HERE */;
println!("Doubled: {:?}", doubled);
```

**Hint:** Use `.map(|&x| x * 2)` and `.collect()`

### Task 1.2: Filter Even Numbers ⭐
**Goal:** Keep only the even numbers

**Expected Output:** `[2, 4, 6, 8, 10]`

```rust
// TODO: Use .filter() to keep only even numbers
let evens = numbers.iter() /* YOUR CODE HERE */;
println!("Evens: {:?}", evens);
```

**Hint:** Use `.filter(|&&x| x % 2 == 0)`

### Task 1.3: Count Long Words ⭐
**Goal:** Count how many words have more than 5 characters

**Expected Output:** `3`

```rust
// TODO: Use .filter() and .count()
let long_word_count = words.iter() /* YOUR CODE HERE */;
println!("Words longer than 5 chars: {}", long_word_count);
```

---

## 📝 Exercise 2: Method Chaining (10 minutes)

### Task 2.1: Process and Filter ⭐⭐
**Goal:** Double all numbers, then keep only those greater than 10

**Expected Output:** `[12, 14, 16, 18, 20]`

```rust
// TODO: Chain .map() and .filter()
let result = numbers.iter() /* YOUR CODE HERE */;
println!("Doubled and > 10: {:?}", result);
```

### Task 2.2: Adult Names ⭐⭐
**Goal:** Find people who are adults (age >= 18), count how many

**Expected Output:** `6`

```rust
// TODO: Filter adults and count them
let adult_count = ages.iter() /* YOUR CODE HERE */;
println!("Number of adults: {}", adult_count);
```

### Task 2.3: Expensive Items ⭐⭐
**Goal:** Find the first item that costs more than $20

**Expected Output:** `Some(23.0)` or similar

```rust
// TODO: Use .find() to get first expensive item
let expensive = prices.iter() /* YOUR CODE HERE */;
println!("First expensive item: {:?}", expensive);
```

---

## 📝 Exercise 3: Advanced Operations (12 minutes)

### Task 3.1: Word Statistics ⭐⭐⭐
**Goal:** For each word, create a tuple of (word, length), but only for words starting with vowels

**Expected Output:** `[("apple", 5), ("elderberry", 10)]`

```rust
// TODO: Filter by first letter, then map to (word, length)
let vowel_words: Vec<_> = words.iter() /* YOUR CODE HERE */;
println!("Vowel words with lengths: {:?}", vowel_words);
```

**Hint:** Check if first character is in "aeiouAEIOU"

### Task 3.2: Price Analysis ⭐⭐⭐
**Goal:** Find the total cost of all items under $20

**Expected Output:** Sum of items under $20

```rust
// TODO: Filter prices < 20, then sum them
let affordable_total: f64 = prices.iter() /* YOUR CODE HERE */;
println!("Total cost of affordable items: ${:.2}", affordable_total);
```

### Task 3.3: Age Groups ⭐⭐⭐
**Goal:** Group ages into categories and count each group
- Minors (< 18): count them
- Adults (18-30): count them  
- Seniors (> 30): count them

```rust
// TODO: Count each age group
let minors = ages.iter() /* YOUR CODE HERE */;
let adults = ages.iter() /* YOUR CODE HERE */;
let seniors = ages.iter() /* YOUR CODE HERE */;

println!("Minors: {}, Adults: {}, Seniors: {}", minors, adults, seniors);
```

---

## 📝 Exercise 4: Complex Chaining Challenge (5 minutes)

### Task 4.1: The Ultimate Chain ⭐⭐⭐⭐
**Goal:** Take numbers 1-20, keep only odd numbers, square them, take first 3, collect to vector

**Expected Output:** `[1, 9, 25]`

```rust
// TODO: Create the ultimate iterator chain
let ultimate: Vec<_> = (1..21) /* YOUR CODE HERE */;
println!("Ultimate result: {:?}", ultimate);
```

**Bonus:** Can you do it all in one chain without intermediate variables?

---

# 🔧 ACT 3: Custom Iterator Implementation (20 minutes)

## 🎯 Your Mission: Build a Custom Iterator

You're going to implement a `Countdown` iterator that counts down from a starting number to 1.

### Example Usage:
```rust
let countdown = Countdown::new(5);
for num in countdown {
    println!("{}", num);  // Prints: 5, 4, 3, 2, 1
}
```

---

## 📝 Exercise 5: Implement Countdown Iterator (15 minutes)

### Step 1: Define the Struct ⭐
```rust
// TODO: Define your Countdown struct
struct Countdown {
    // What fields do you need to track the current state?
    // HINT: You need to know where you are now
    /* YOUR CODE HERE */
}
```

### Step 2: Constructor ⭐
```rust
impl Countdown {
    // TODO: Create a new Countdown starting from 'start'
    fn new(start: u32) -> Countdown {
        /* YOUR CODE HERE */
    }
}
```

### Step 3: Implement Iterator Trait ⭐⭐
```rust
impl Iterator for Countdown {
    type Item = u32;  // We return u32 numbers
    
    // TODO: Implement the next() method
    // HINT: Return Some(current_number) or None when done
    fn next(&mut self) -> Option<Self::Item> {
        /* YOUR CODE HERE */
    }
}
```

### Step 4: Test Your Iterator ⭐⭐⭐
```rust
fn main() {
    println!("Testing Countdown iterator:");
    
    // Test 1: Basic iteration
    println!("Countdown from 5:");
    for num in Countdown::new(5) {
        println!("{}", num);
    }
    
    // Test 2: Use with built-in methods
    let countdown_vec: Vec<u32> = Countdown::new(3).collect();
    println!("Collected countdown: {:?}", countdown_vec);
    
    // Test 3: Chain with other iterator methods
    let sum: u32 = Countdown::new(4).map(|x| x * x).sum();
    println!("Sum of squares from countdown(4): {}", sum);
}
```

---

## 🏆 Bonus Challenges (5 minutes)

### Bonus 1: Fizz Iterator ⭐⭐⭐⭐
Create an iterator that produces "Fizz", "Buzz", "FizzBuzz", or the number as a string

### Bonus 2: Step Iterator ⭐⭐⭐⭐
Create an iterator that goes from `start` to `end` in steps of `step_size`

### Bonus 3: Fibonacci Iterator ⭐⭐⭐⭐⭐
Create an infinite Fibonacci sequence iterator

---

## 📚 Solution Templates

<details>
<summary>🔍 Click to reveal solution hints (try first!)</summary>

### Exercise 1 Solutions:
```rust
// 1.1: Double numbers
let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();

// 1.2: Filter evens  
let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();

// 1.3: Count long words
let long_word_count = words.iter().filter(|&word| word.len() > 5).count();
```

### Exercise 2 Solutions:
```rust
// 2.1: Process and filter
let result: Vec<i32> = numbers.iter().map(|&x| x * 2).filter(|&x| x > 10).collect();

// 2.2: Adult count
let adult_count = ages.iter().filter(|&&age| age >= 18).count();

// 2.3: First expensive
let expensive = prices.iter().find(|&&price| price > 20.0);
```

### Custom Iterator Solution:
```rust
struct Countdown {
    current: u32,
}

impl Countdown {
    fn new(start: u32) -> Countdown {
        Countdown { current: start }
    }
}

impl Iterator for Countdown {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > 0 {
            let result = self.current;
            self.current -= 1;
            Some(result)
        } else {
            None
        }
    }
}
```

</details>

---

## 🎉 Workshop Wrap-Up

### What You've Learned:
- ✅ Iterator methods: `map`, `filter`, `collect`, `find`, `count`, `sum`
- ✅ Method chaining for elegant data processing
- ✅ Custom iterator implementation with `next()` method
- ✅ When iterators shine vs traditional approaches

### Key Takeaways:
1. **Iterators are zero-cost abstractions** - they compile to the same assembly as hand-written loops
2. **The `next()` method is the heart** of every iterator - implement it and get all other methods for free
3. **Iterators make intent clear** - your code reads like what you want to accomplish
4. **Safety first** - no manual indexing means no bounds errors

### Next Steps:
- Practice with more complex data structures
- Explore `Iterator` trait methods you didn't use today
- Try implementing other iterator patterns (infinite sequences, adapters)
- Use iterators in your real projects!

**Happy iterating! 🦀**