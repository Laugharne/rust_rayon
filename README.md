# Parallel Programming with Rayon

> **Source: [Day 50: Parallel Programming with Rayon | by Tech Insights Hub - Freedium](https://freedium.cfd/https://blog.cubed.run/day-50-parallel-programming-with-rayon-a2c6def00459)**

Rayon empowers developers to write highly efficient, **data-parallel code** while maintaining Rust's guarantees of memory safety and concurrency. Let's delve deeper into the intricacies of Rayon, its usage, and practical implementations.

## What Is the Rayon Crate and Why Use It?

The Rayon crate is a lightweight, **data-parallelism library** designed to make writing **concurrent applications in Rust** simpler and more intuitive. While traditional concurrency often involves managing threads manually (as we saw in Day 49), Rayon abstracts this complexity, allowing you to focus solely on the logic of your application.

## Key Features of Rayon

- **Automatic Thread Management**: Rayon automatically manages thread pools, balancing workloads effectively.
- **Easy Integration**: Its API seamlessly integrates with Rust's standard collections and iterators.
- **Safe Parallelism**: Rayon ensures safety by leveraging Rust's ownership model and preventing race conditions.

## Getting Started with Rayon

### Installing Rayon

To use Rayon in your Rust project, add it to your `Cargo.toml` dependencies:

```toml
[dependencies]
rayon = "1.7"
```

Run `cargo build` to download and compile the dependency.

### Parallel Iterators: The Heart of Rayon

Rayon's most powerful feature is its support for **parallel iterators**, which allow you to process collections concurrently with minimal code changes.

#### Example: Converting Sequential Iteration to Parallel Iteration

Here's how we can transform sequential operations into parallel ones using Rayon:

**Sequential Iteration (Without Rayon):**

```rust
let numbers: Vec<i32> = (1..=100).collect();
let sum: i32 = numbers.iter().map(|&x| x * 2).sum();
println!("Sum: {}", sum);
```

**Parallel Iteration (With Rayon):**

```rust
use rayon::prelude::*;

let numbers: Vec<i32> = (1..=100).collect();
let sum: i32 = numbers.par_iter().map(|&x| x * 2).sum();
println!("Sum: {}", sum);
```


#### Key Changes:

- Replace `.iter()` with `.par_iter()`.
- The rest of the logic remains unchanged, but the computation is now parallelized.

## Real-World Applications of Rayon

### 1. Parallel Data Processing

When working with large datasets, processing them sequentially can be time-consuming. Rayon excels in such scenarios by dividing workloads across multiple threads.

**Example: Processing a Large Dataset**

```rust
use rayon::prelude::*;

fn process_data(data: &[i32]) -> Vec<i32> {
    data.par_iter()
        .map(|&x| x * x)
        .filter(|&x| x % 2 == 0)
        .collect()
}

let data: Vec<i32> = (1..=1_000_000).collect();
let result = process_data(&data);
println!("Processed {} items", result.len());
```

In this example:

- Each element is squared and filtered for even values.
- Rayon handles thread distribution efficiently, minimizing processing time.

### 2. Parallel Sorting

Sorting large collections can benefit significantly from parallelization. Rayon provides the `par_sort` method for collections.

**Example: Sorting Numbers in Parallel**

```rust
use rayon::prelude::*;

let mut numbers: Vec<i32> = (1..=1_000_000).rev().collect();
numbers.par_sort();
println!("First 10 numbers: {:?}", &numbers[..10]);
```

By using `par_sort`, the sorting operation is distributed across threads, resulting in faster execution for large datasets.

### 3. Custom Parallelism Logic

Rayon also allows developers to implement custom parallelization logic using the `par_iter()` API.

**Example: Parallel Matrix Multiplication**

```rust
use rayon::prelude::*;

fn matrix_multiply(a: &[Vec<i32>], b: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let n = a.len();
    let m = b[0].len();
    let p = b.len();

    let b_transposed: Vec<Vec<i32>> = (0..m)
        .map(|j| (0..p).map(|i| b[i][j]).collect())
        .collect();

    (0..n)
        .into_par_iter()
        .map(|i| {
            (0..m)
                .map(|j| {
                    (0..p).map(|k| a[i][k] * b_transposed[j][k]).sum()
                })
                .collect()
        })
        .collect()
}

let a = vec![vec![1, 2, 3], vec![4, 5, 6]];
let b = vec![vec![7, 8], vec![9, 10], vec![11, 12]];
let result = matrix_multiply(&a, &b);
println!("{:?}", result);
```

### Exercise: Implement Parallel Data Processing

As an exercise, use Rayon to parallelize a computation on a large dataset. Write a function to compute the frequency of each word in a large text file.

**Hints:**

1. Split the text file into chunks for parallel processing.
2. Use `par_iter` for concurrency.
3. Merge intermediate results into a single frequency map.

### Best Practices for Using Rayon

1. **Understand Workload Characteristics**: Parallelization overhead can outweigh benefits for small datasets.
2. **Avoid Side Effects**: Ensure that operations within parallel iterators are thread-safe and free from side effects.
3. **Profile Your Code**: Use tools like `cargo flamegraph` to measure performance improvements.

Rayon significantly simplifies **parallel programming in Rust**, enabling developers to achieve higher performance with less effort. By leveraging **parallel iterators**, **automatic thread management**, and **data safety**, Rayon makes it easier to harness the power of multi-core processors.

--------

You will find in this repository the **result of this exercise**, in `rust_rayon` directory...

```bash
.
├── rust_rayon
│   ├── src
│   │   └── main.rs
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── lorem.txt
│   └── test.txt
└── README.md
```
