use std::collections::{HashMap, HashSet};

fn anagrams(words: HashSet<&str>) -> HashMap<String, Vec<String>> {

    // Создаем временную мапу для хранения анаграмм
    let mut anagram_map: HashMap<String, Vec<String>> = HashMap::new();

    for word in words{
         // Приводим слово к нижнему регистру
        let lower_word = word.to_lowercase();
        // Сортируем буквы в слове
        let mut sort_chars: Vec<char> = word.chars().collect();
        sort_chars.sort_unstable();
        let sorted_word = sort_chars.iter().collect();
        // Добавляем слово в соответствующую группу анаграмм
        anagram_map.entry(sorted_word).or_insert(Vec::new()).push(lower_word.clone());
    }

    // Фильтруем множества, содержащие только одно слово, и сортируем каждый массив анаграмм
    let mut sors_anagrams: HashMap<String, Vec<String>> = HashMap::new();

    for (_, mut anagrams) in anagram_map{
        if anagrams.len() > 1{
            // Сортируем массив анаграмм по возрастанию
            anagrams.sort();
            // В качестве ключа берем первое слово из отсортированного массива
            sors_anagrams.insert(anagrams[0].clone(), anagrams);
        }
    }
    sors_anagrams
}


fn main() {
    let words =[ "листок", "слиток", "столик", "кот", "ток", "кто", "пятак", "пятка", "тяпка",];

    let set: HashSet<&str> = HashSet::from(words);
    let anagrams = anagrams(set);

    for (key, value) in anagrams{
        println!("{}: {:?}", key, value)
    }


}