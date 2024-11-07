use rand::Rng;  // Для генерації випадкових чисел

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..99)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut idx1 = 0;
    let mut idx2 = 0;

    // Перебираємо пари сусідніх елементів
    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            idx1 = i;
            idx2 = i + 1;
        }
    }

    (min_sum, idx1, idx2)
}

fn print_output(data: &[i32], i1: usize, i2: usize) {
    let indexes_str = (0..data.len())
        .map(|i| if i == i1 || i == i2 { "\\__" } else { "  " })
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    println!("indexes: {}", indexes_str.join(" "));
    println!("data:  [{}]", data.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "));

    let min_str = format!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[i1], data[i2], data[i1] + data[i2], i1, i2
    );

    println!("{}", min_str);
}
#[test]
fn main() {
    let random_vector = gen_random_vector(20);  // Генерація випадкового вектора

    // Знаходимо мінімальну пару
    let (_min_sum, i1, i2) = min_adjacent_sum(&random_vector);

    // Викликаємо print_output, передаючи індекси мінімальної пари
    print_output(&random_vector, i1, i2);
}

