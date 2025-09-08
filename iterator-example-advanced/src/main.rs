fn main() {
    println!("🦀 Advanced Iterator Workshop - Custom Iterators\n");

    // 1. THE ITERATOR TRAIT - What makes an iterator tick
    println!("1. THE ITERATOR TRAIT");
    println!("Every iterator must implement:");
    println!("  trait Iterator {{");
    println!("      type Item;");
    println!("      fn next(&mut self) -> Option<Self::Item>;");
    println!("  }}");
    println!("The next() function is THE CORE of every iterator!\n");

    // 2. SIMPLE COUNTER - Our first custom iterator
    println!("2. SIMPLE COUNTER - Custom iterator from scratch");
    let mut counter = Counter::new(5);
    print!("Counter(5): ");
    while let Some(n) = counter.next() {
        print!("{} ", n);
    }
    println!(" // Output: 1 2 3 4 5\n");

    // 3. USING WITH BUILT-IN METHODS - It's a real iterator!
    println!("3. COUNTER WITH BUILT-IN METHODS");
    let doubled: Vec<i32> = Counter::new(3).map(|x| x * 2).collect(); // Output: [2, 4, 6]
    let sum: i32 = Counter::new(4).sum(); // Output: 10 (1+2+3+4)
    println!("Counter(3) doubled: {:?}", doubled);
    println!("Counter(4) sum: {}\n", sum);

    // 4. FIBONACCI ITERATOR - More complex state
    println!("4. FIBONACCI ITERATOR - Complex state management");
    let fibs: Vec<u64> = Fibonacci::new().take(8).collect(); // Output: [1, 1, 2, 3, 5, 8, 13, 21]
    println!("First 8 Fibonacci numbers: {:?}\n", fibs);

    // 5. STEP ITERATOR - Customizable behavior
    println!("5. STEP ITERATOR - Customizable step size");
    let by_twos: Vec<i32> = StepRange::new(0, 10, 2).collect(); // Output: [0, 2, 4, 6, 8]
    let by_threes: Vec<i32> = StepRange::new(1, 15, 3).collect(); // Output: [1, 4, 7, 10, 13]
    println!("0 to 10 by 2s: {:?}", by_twos);
    println!("1 to 15 by 3s: {:?}\n", by_threes);

    // 6. TURNING STRUCTS INTO ITERATORS - IntoIterator trait
    println!("6. TURNING STRUCTS INTO ITERATORS");
    let my_vec = MyVec { data: vec![10, 20, 30] };
    let doubled: Vec<i32> = my_vec.into_iter().map(|x| x * 2).collect(); // Output: [20, 40, 60]
    println!("MyVec doubled: {:?}\n", doubled);

    // 7. DEMONSTRATING LAZINESS - Iterators are lazy!
    println!("7. DEMONSTRATING LAZINESS");
    println!("Creating iterator chain...");
    let _lazy = Counter::new(1000000)  // This doesn't do work yet!
        .map(|x| { println!("Processing {}", x); x * 2 })
        .filter(|&x| x > 10);
    println!("Iterator created, but no work done!");
    println!("Only when we collect/consume do we see the work:");
    let result: Vec<i32> = Counter::new(3)
        .map(|x| { print!("Processing {} ", x); x * 2 })
        .filter(|&x| x > 2)
        .collect(); // Output: Processing 1 Processing 2 Processing 3 then [4, 6]
    println!(" -> {:?}\n", result);

    // 8. ITERATOR ADAPTERS VS CONSUMERS
    println!("8. ITERATOR ADAPTERS VS CONSUMERS");
    println!("ADAPTERS (lazy): map, filter, take, skip, enumerate...");
    println!("CONSUMERS (eager): collect, for_each, sum, count, find...");
    let _adapter_chain = (1..10).map(|x| x * 2).filter(|&x| x > 5); // No work yet!
    let consumer_result: Vec<i32> = (1..5).map(|x| x * 2).collect(); // Work happens here!
    println!("Consumer triggered work: {:?}\n", consumer_result);
}

// 1. SIMPLE COUNTER ITERATOR
struct Counter {
    current: i32,
    max: i32,
}

impl Counter {
    fn new(max: i32) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current) // Return 1, 2, 3, ..., max
        } else {
            None // Iterator is exhausted
        }
    }
}

// 2. FIBONACCI ITERATOR
struct Fibonacci {
    current: u64,
    next_val: u64,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { current: 1, next_val: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        
        // Calculate the next Fibonacci number
        let new_next = self.current + self.next_val;
        self.current = self.next_val;
        self.next_val = new_next;
        
        Some(result) // Infinite iterator - never returns None!
    }
}

// 3. STEP RANGE ITERATOR
struct StepRange {
    current: i32,
    end: i32,
    step: i32,
}

impl StepRange {
    fn new(start: i32, end: i32, step: i32) -> StepRange {
        StepRange { 
            current: start, 
            end, 
            step 
        }
    }
}

impl Iterator for StepRange {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current;
            self.current += self.step;
            Some(result)
        } else {
            None
        }
    }
}

// 4. MAKING A STRUCT ITERABLE WITH IntoIterator
struct MyVec {
    data: Vec<i32>,
}

struct MyVecIntoIter {
    data: Vec<i32>,
    index: usize,
}

impl IntoIterator for MyVec {
    type Item = i32;
    type IntoIter = MyVecIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        MyVecIntoIter {
            data: self.data,
            index: 0,
        }
    }
}

impl Iterator for MyVecIntoIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let result = self.data[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}
