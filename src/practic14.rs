#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    a: Point,  // Верхній лівий кут
    b: Point,  // Нижній правий кут
}

impl Rectangle {
    // Обчислюємо площу прямокутника
    fn area(&self) -> i32 {
        (self.b.x - self.a.x).abs() * (self.a.y - self.b.y).abs()
    }

    // Перевіряємо, чи два прямокутники перекриваються
    fn overlap(&self, other: &Rectangle) -> Option<Rectangle> {
        let x_overlap = std::cmp::max(0, std::cmp::min(self.b.x, other.b.x) - std::cmp::max(self.a.x, other.a.x));
        let y_overlap = std::cmp::max(0, std::cmp::min(self.a.y, other.a.y) - std::cmp::max(self.b.y, other.b.y));

        if x_overlap > 0 && y_overlap > 0 {
            // Якщо є перетин, створюємо новий прямокутник для перетину
            Some(Rectangle {
                a: Point {
                    x: std::cmp::max(self.a.x, other.a.x),
                    y: std::cmp::min(self.a.y, other.a.y),
                },
                b: Point {
                    x: std::cmp::min(self.b.x, other.b.x),
                    y: std::cmp::max(self.b.y, other.b.y),
                },
            })
        } else {
            None
        }
    }
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut total_area = 0;

    // Обчислюємо сумарну площу всіх прямокутників
    for rect in xs.iter() {
        total_area += rect.area();
    }

    let mut overlap_area = 0;

    // Для кожної пари прямокутників перевіряємо перетини
    for i in 0..xs.len() {
        for j in i + 1..xs.len() {
            if let Some(overlap) = xs[i].overlap(&xs[j]) {
                overlap_area += overlap.area();
            }
        }
    }

    // Повертаємо фактичну площу, віднімаючи площу перетинів
    total_area - overlap_area
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_occupied_test() {
        let data = test_data();
        let occupied = area_occupied(&data);
        assert_eq!(occupied, 60); // Фактична площа повинна бути 60
    }
}








