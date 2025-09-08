mod traditional;
mod iterators;

fn main() {
    println!("🦀 Iterator Workshop - Side by Side Comparison\n");
    println!("Same tasks, two approaches - see the difference!\n");
    println!("{}", "=".repeat(60));

    // Show traditional approach first
    traditional::demo_traditional();
    
    println!("\n{}", "=".repeat(60));
    
    // Then show iterator approach
    iterators::demo_iterators();
    
    println!("\n{}", "=".repeat(60));
    println!("\n💡 KEY INSIGHT:");
    println!("Both approaches produce IDENTICAL results and performance!");
    println!("But iterators are safer, clearer, and more maintainable.");
    
    // Performance demonstration
    println!("\n⚡ PERFORMANCE COMPARISON:");
    let large_vec: Vec<i32> = (1..100000).collect();
    
    use std::time::Instant;
    
    // Traditional way
    let start = Instant::now();
    let mut sum_traditional: i64 = 0;
    for i in 0..large_vec.len() {
        if large_vec[i] % 2 == 0 {
            let val = large_vec[i] as i64;
            sum_traditional += val * val;
        }
    }
    let traditional_time = start.elapsed();
    
    // Iterator way  
    let start = Instant::now();
    let sum_iterator: i64 = large_vec.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| (x as i64) * (x as i64))
        .sum();
    let iterator_time = start.elapsed();
    
    println!("Traditional: {} (took: {:?})", sum_traditional, traditional_time);
    println!("Iterator:    {} (took: {:?})", sum_iterator, iterator_time);
    println!("Both sums equal: {}", sum_traditional == sum_iterator);
    println!("\nIterators compile to the SAME assembly code! Zero-cost abstractions! 🎉");
}
