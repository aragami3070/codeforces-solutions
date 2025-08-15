// D. Три активности
// Приближаются зимние каникулы. Они будут длиться n дней.
//
// Во время каникул Монокарп с друзьями хочет ровно по одному разу:
// - пойти кататься на лыжах;
// - посмотреть фильм в кинотеатре;
// - поиграть в настольные игры.
//
// Монокарп знает, что в i-й день ровно a_i друзей соберутся на катание на лыжах,
// b_i друзей — на просмотр фильма и c_i друзей — на настольные игры.
//
// Монокарп также знает, что он не может заниматься более чем одной активностью в течение одного дня.
//
// Поэтому он просит вас помочь ему выбрать три различных дня x,y,z таким образом,
// чтобы общее количество друзей, присоединившихся к нему на активности (a_x + b_y + c_z),
// было максимальным.
//
// Входные данные
//
// Первая строка содержит одно целое число t (1≤t≤10^4) — количество наборов входных данных.
//
// Первая строка каждого теста содержит одно целое число n (3≤n≤10^5) — продолжительность
// зимних каникул в днях.
//
// Вторая строка содержит n целых чисел a_1,a_2,…,a_n (1≤ai≤10^8) — количество друзей,
// которые присоединятся к Монокарпу для катания на лыжах в i-й день.
//
// Третья строка содержит n целых чисел b1,b2,…,bn (1≤bi≤10^8) — количество друзей,
// которые присоединятся к Монокарпу для просмотра фильма в i-й день.
//
// Сумма n по всем тестам не превышает 10^5.
//
// Выходные данные
// На каждый набор входных данных выведите одно целое число — максимальное суммарное
// количество друзей, которые могут присоединиться к Монокарпу за три различных дня.
//
// Пример
//
// Входные данные
// 4
// 3
// 1 10 1
// 10 1 1
// 1 1 10
// 4
// 30 20 10 1
// 30 5 15 20
// 30 25 10 10
// 10
// 5 19 12 3 18 18 6 17 10 13
// 15 17 19 11 16 3 11 17 17 17
// 1 17 18 10 15 8 17 3 13 12
// 10
// 17 5 4 18 12 4 11 2 16 16
// 8 4 14 19 3 12 6 7 5 16
// 3 4 8 11 10 8 10 2 20 3
//
// Выходные данные
// 30
// 75
// 55
// 56

type Index = u16;

#[derive(Debug)]
struct Triple {
    pub first: (u32, Index),
    pub second: (u32, Index),
    pub third: (u32, Index),
}

impl Triple {
    fn new() -> Triple {
        Triple {
            first: (0, 0),
            second: (0, 0),
            third: (0, 0),
        }
    }

    fn add(&mut self, new_max: (u32, Index)) {
        self.first = self.second;
        self.second = self.third;
        self.third = new_max;
        self.sort()
    }

    fn sort(&mut self) {
        if self.first > self.second {
            swap(&mut self.first, &mut self.second);
        }
        if self.second > self.third {
            swap(&mut self.second, &mut self.third);

            if self.first > self.second {
                swap(&mut self.first, &mut self.second);
            }
        }
    }

    fn is_full(&mut self) -> bool {
        !(self.first.0 == 0 || self.second.0 == 0 || self.third.0 == 0)
    }
}

use std::{io, mem::swap};

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed input");

    let t: u16 = input.trim().parse().expect("t must be a number");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");

        let _n: u32 = input.trim().parse().expect("n must be a number");

        let mut max_ski = Triple::new();
        let mut max_film = Triple::new();
        let mut max_game = Triple::new();

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");

        for (index, s) in input.split_whitespace().enumerate() {
            let num = s.parse::<u32>().expect("parse error");
            if num > max_ski.third.0
                || num > max_ski.second.0
                || num > max_ski.first.0
                || !max_ski.is_full()
            {
                max_ski.add((num, index as u16));
            };
        }

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");

        for (index, s) in input.split_whitespace().enumerate() {
            let num = s.parse::<u32>().expect("parse error");
            if num > max_film.third.0
                || num > max_film.second.0
                || num > max_film.first.0
                || !max_film.is_full()
            {
                max_film.add((num, index as u16));
            };
        }

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");

        for (index, s) in input.split_whitespace().enumerate() {
            let num = s.parse::<u32>().expect("parse error");
            if num > max_game.third.0
                || num > max_game.second.0
                || num > max_game.first.0
                || !max_game.is_full()
            {
                max_game.add((num, index as u16));
            };
        }

        let ski = vec![max_ski.first, max_ski.second, max_ski.third];
        let film = vec![max_film.first, max_film.second, max_film.third];
        let game = vec![max_game.first, max_game.second, max_game.third];

        let mut result = 0;

        for s in ski {
            for f in &film {
                for g in &game {
                    if s.1 != f.1 && f.1 != g.1 && g.1 != s.1 {
                        let sum = s.0 + f.0 + g.0;
                        if sum > result {
                            result = sum
                        }
                    }
                }
            }
        }
        println!("{result}")
    }
}
