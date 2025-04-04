// Вычислите модуль вектора просуммировав квадраты его координат
// и вычислив кввдратный корень полученного значения. Используйте метод `sqrt()` для вычисления
// корня, следующим образом: v.sqrt().

fn magnitude(vector: &[f64]) -> f64 {
    let sum: f64 = vector.iter().map(|&x| x * x).sum();
    sum.sqrt()
}

// Нормализуйте вектор вычислив его модуль и разделив все его координаты на 
// этот модудль.

fn normalize(vector: &mut [f64]) {
    let mag = magnitude(vector);
    for component in vector.iter_mut() {
        *component /= mag;
    }
}

// Используйте эту функцию main для проверки своей работы.

fn main() {
    println!("Модуль единичного вектора: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Модуль {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Модуль {v:?} после нормализации: {}", magnitude(&v));
}