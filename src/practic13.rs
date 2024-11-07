fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();

    // Перевірка на можливість рівномірного розподілу
    if total % n as u32 != 0 {
        return 0; // Якщо не можна рівномірно розподілити
    }

    let average = total / n as u32;
    let mut moves = 0;
    let mut diff = 0;

    // Підрахунок мінімальних переміщень
    for &shipment in shipments.iter() {
        diff += shipment as i32 - average as i32;
        moves += diff.abs() as usize;
    }

    moves
}
#[allow(dead_code)]
fn gen_shipments(n: usize) -> Vec<u32> {
    let shipments = vec![n as u32; n];
    shipments
}
#[test]
fn main() {
    let shipments1 = vec![8, 2, 2, 4, 4];
    let shipments2 = vec![9, 3, 7, 2, 9];

    // Обчислюємо мінімальні переміщення
    let result1 = count_permutation(&shipments1);
    let result2 = count_permutation(&shipments2);

    // Виводимо результати
    println!("Minimal moves for shipments1: {}", result1);
    println!("Minimal moves for shipments2: {}", result2);
}
