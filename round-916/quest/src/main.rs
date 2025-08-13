// C. Квесты
//
// Монокарп играет в компьютерную игру. Чтобы повысить уровень персонажа,
// он может выполнять квесты. В игре есть n квестов, пронумерованных от 1 до n.
//
// Монокарп может выполнять квесты согласно следующим правилам:
//
// - 1-й квест всегда доступен для выполнения;
// - i-й квест доступен для выполнения, если каждый квест j<i был выполнен хотя бы раз.
//
// Обратите внимание, что Монокарп может выполнять один и тот же квест несколько раз.
//
// За каждое выполнение персонаж получает некоторое количество опыта:
//
// - за первое выполнение i-го квеста он получает a_i очков опыта;
// - за каждое последующее выполнение i-го квеста он получает b_i очков опыта.
//
// Монокарп очень занятой человек, поэтому у него есть свободное время,
// чтобы выполнить не более k квестов. Ваша задача — посчитайте максимально возможный
// суммарный опыт, который Монокарп может получить, если он может выполнить не более k квестов.
//
// Входные данные
// 
// Первая строка содержит одно целое число t (1≤t≤10^4) — количество наборов входных данных.
// 
// Первая строка каждого набора содержит два целых числа n и k (1≤n≤2⋅10^5 ; 1≤k≤2⋅105)
// — количество квестов и максимальное количество квестов,
// которые Монокарп может выполнить, соответственно.
// 
// Вторая строка содержит n целых чисел a_1,a_2,…,a_n (1≤a_i≤10^3).
// Третья строка содержит n целых чисел b_1,b_2,…,b_n (1≤b_i≤10^3).
// 
// Дополнительное ограничение на входные данные: сумма n по всем наборам входных
// данных не превосходит 2⋅10^5.
// 
// Выходные данные
// Для каждого набора входных данных выведите одно целое число — максимально возможный
// суммарный опыт, который Монокарп может получить, если он может выполнить не более k квестов.
// 
// Пример
// Входные данные
//
// 4
// 4 7
// 4 3 1 2
// 1 1 1 1
// 3 2
// 1 2 5
// 3 1 8
// 5 5
// 3 2 4 1 4
// 2 3 1 4 7
// 6 4
// 1 4 5 4 5 10
// 1 5 1 2 5 1
// 
// Выходные данные
//
// 13
// 4
// 15
// 15

use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed input");

    let t: u16 = input.trim().parse().expect("t will be a number (u16)");

    for _ in 0..t {
        input.clear();

        io::stdin().read_line(&mut input).expect("Failed input");

        let pair_input = input.split_once(' ').expect("Failed input");
        let n: u32 = pair_input.0.trim().parse().expect("n will be a number");
        let k: u32 = pair_input.1.trim().parse().expect("n will be a number");

        input.clear();

        io::stdin().read_line(&mut input).expect("Failed input");
        let first_try: Vec<u16> = input
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();

        input.clear();

        io::stdin().read_line(&mut input).expect("Failed input");
        let other_try: Vec<u16> = input
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();

        let mut max_other_try: u64 = other_try[0] as u64;

        let max_quests = match n < k {
            true => n,
            false => k,
        };

        let mut result: u64 = 0;

        let mut first_try_sum: u64 = 0;

        for i in 0..max_quests {
            if max_other_try < other_try[i as usize] as u64 {
                max_other_try = other_try[i as usize] as u64;
            }

            first_try_sum += first_try[i as usize] as u64;

            let calc: u64 = first_try_sum + max_other_try * ((k - i - 1) as u64);

            result = match calc > result {
                true => calc,
                false => result,
            }
        }
        println!("{result}")
    }
}
