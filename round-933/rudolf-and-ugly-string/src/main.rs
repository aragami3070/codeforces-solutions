// C. Рудольф и некрасивая строка
//
// У Рудольфа есть строка s длины n. Рудольф считает, что строка s является некрасивой,
// если содержит в качестве подстроки† хотя бы одну строку «pie» или хотя бы одну строку «map»,
// в противном случае строка s будет красивой.
//
// Например, «ppiee», «mmap», «dfpiefghmap» — некрасивые строки, а «mathp», «ppiiee» — красивые строки.
//
// Рудольф хочет сократить строку s, удалив некоторые символы, чтобы она стала красивой.
//
// Главный герой не любит напрягаться, поэтому просит вас сделать строку красивой,
// удалив при этом минимальное количество символов.
// Он может удалять символы из любых позиций в строке (а не только с начала/конца строки).
//
// Строка a является подстрокой b, если в строке b существует последовательный
// отрезок символов равный a.
//
// Входные данные
// Первая строка содержит одно целое число t (1≤t≤104) — количество наборов
// входных данных. Далее следуют описания наборов.
//
// Первая строка набора содержит одно целое число n (1≤n≤106) — длину строки s.
//
// Следующая строка набора содержит строку s длины n.
// Строка s состоит исключительно из строчных латинских букв.
//
// Сумма n по всем наборам входных данных не превосходит 106.
//
// Выходные данные
// Для каждого набора входных данных выведите единственное целое число —
// минимальное количество символов, которые нужно удалить, чтобы строка s стала красивой.
// Если строка изначально красивая, то выведите 0 .
//
// Пример:
// Входные данные:
//
// 6
// 9
// mmapnapie
// 9
// azabazapi
// 8
// mappppie
// 18
// mapmapmapmapmapmap
// 1
// p
// 11
// pppiepieeee
//
// Выходные данные:
//
// 2
// 0
// 2
// 6
// 0
// 2

use std::io;
const ALPHABET_LEN: u16 = 256;

fn pre_bm_bc(mut table: Vec<u32>, pattern: &str) -> Vec<u32> {
    for i in 0..ALPHABET_LEN {
        table[i as usize] = pattern.len() as u32;
    }
    for i in 0..pattern.len() - 1 {
        table[pattern.as_bytes()[i as usize] as usize] = (pattern.len() - 1 - i) as u32;
    }
    table
}

fn is_prefix(pattern: &String, pos: &u32) -> bool {
    let suffix_len = pattern.len() - (*pos as usize);

    for i in 0..suffix_len {
        if pattern.as_bytes()[i as usize] != pattern.as_bytes()[(i + 1) as usize] {
            return false;
        }
    }
    true
}

fn suffix_len(pattern: &String, pos: &u32) -> u32 {
    let mut len = 0;
    loop {
        if len >= *pos
            || pattern.as_bytes()[(pos - len) as usize]
                != pattern.as_bytes()[(pattern.len() - 1 - (len) as usize) as usize]
        {
            break;
        }
        len += 1;
    }
    len
}

fn pre_bm_gc(mut table: Vec<u32>, pattern: &String) -> Vec<u32> {
    let mut last_prefix_index = 1;

    // Этап 1: находим границы совпадений
    for pos in pattern.len() - 1..0 {
        if is_prefix(pattern, &((pos + 1) as u32)) {
            last_prefix_index = pos + 1;
        }
        table[pos] = (pattern.len() - 1 - pos + last_prefix_index) as u32;
    }

    // Этап 2: Заполняем таблицу хороших суффиксов
    for pos in 0..pattern.len() - 1 {
        let suf_len = suffix_len(pattern, &(pos as u32)) as usize;

        if pattern.as_bytes()[pos - suf_len] != pattern.as_bytes()[pattern.len() - 1 - suf_len] {
            table[pattern.len() - 1 - suf_len] = (pattern.len() - pos + suf_len) as u32 - 1;
        }
    }
    table
}

fn boyer_moore(text: &String, pattern: &str) -> Vec<u32>{
    let mut bad_char: Vec<u32> = vec![0; ALPHABET_LEN.into()];
    let mut good_suf: Vec<u32> = vec![0; pattern.len()];

    bad_char = pre_bm_bc(bad_char, pattern);
    good_suf = pre_bm_gc(good_suf, &pattern.to_string());

    let mut result: Vec<u32> = Vec::new();

    // Позиция в тексте
    let mut text_index = pattern.len() - 1;

    let mut flag = false;

    while text_index < text.len() {
        // Позиция в шаблоне
        let mut pattern_index = pattern.len() - 1;

        // Сравниваем спарва налево
        while text.as_bytes()[text_index] == pattern.as_bytes()[pattern_index]
        {
            if text_index == 0 || pattern_index == 0 {
                flag = true;
                break;
            }
            text_index -= 1;
            pattern_index -= 1;
        }

        // Найдено совпадение
        if flag {
            result.push(text_index as u32);
            text_index += pattern.len();
            flag = false;
            continue;
        }

        // Сдвиг по максмимальному из двух правил
        text_index += std::cmp::max(
            bad_char[text.as_bytes()[text_index] as usize],
            good_suf[pattern_index],
        ) as usize;
    }

	result
}

fn main() {
}
