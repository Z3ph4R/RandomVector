use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| rng.gen_range(10..100))
        .collect()
}

fn min_adjacent_sum(data: &[i32]) -> i32 {
    let mut min_sum = i32::MAX;
    for window in data.windows(2) {
        let sum = window[0] + window[1];
        if sum < min_sum {
            min_sum = sum;
        }
    }
    min_sum
}

fn print_vector(data: &[i32]) {
    println!("Масив: {:?}", data);
    let min_sum = min_adjacent_sum(data);
    println!("Мінімальна пара сум: {}", min_sum);
}

fn main() {
    let random_vector = gen_random_vector(20);
    print_vector(&random_vector);
}
