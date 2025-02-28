// Вычислите модуль вектора просуммировав квадраты его координат
// и вычислив квадратный корень полученного значения. Используйте метод `sqrt()` для вычисления
// корня, следующим образом: v.sqrt().


fn magnitude(arr : &[f64; 3]) -> f64 {
    let mut sq_sum : f64 = 0.0;
    for coord in *arr {
        sq_sum += coord * coord;
    }
    sq_sum.sqrt()
}

// Нормализуйте вектор вычислив его модуль и разделив все его координаты на 
// этот модудль.


fn normalize(arr : &mut [f64; 3]) {
    let module = magnitude(arr);
    for idx in 0..3 {
        arr[idx] /= module;
    }
}

pub fn demonstrate() {
    println!("--- Task 4 ---");
    println!("Модуль единичного вектора: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Модуль {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Модуль {v:?} после нормализации: {}", magnitude(&v));
}
