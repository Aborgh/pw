use crate::helper::output::is_clean_break;
use crate::helper::phonetics::{contains_vowel, ends_with_vowel, get_phonetics};
use rand::Rng;
use std::collections::{HashMap, HashSet};

pub fn generate_random_word(min_length: usize, max_length: usize, rng: &mut impl Rng) -> String {
    if min_length <= 3 && max_length <= 3 {
        let word = generate_short_english_word(max_length, rng);

        return if word.len() >= min_length && word.len() <= max_length {
            word
        } else {
            generate_exact_length_word(min_length, max_length, rng)
        };
    }

    let candidates = (0..15)
        .map(|_| {
            let algorithm_choice = rng.random_range(0..4);

            let word = match algorithm_choice {
                0 => generate_phoneme_based_word(min_length, max_length, rng),
                1 => generate_syllable_based_word(min_length, max_length, rng),
                2 => generate_pattern_based_word(min_length, max_length, rng),
                _ => generate_common_english_word(min_length, max_length, rng),
            };

            (word.clone(), score_english_word(&word))
        })
        .collect::<Vec<(String, f64)>>();

    let result = if !candidates.is_empty() {
        candidates
            .into_iter()
            .max_by(|(_, score1), (_, score2)| score1.partial_cmp(score2).unwrap())
            .map(|(word, _)| word)
            .unwrap()
    } else {
        generate_exact_length_word(min_length, max_length, rng)
    };

    if result.len() < min_length || result.len() > max_length {
        return generate_exact_length_word(min_length, max_length, rng);
    }

    result
}
fn generate_exact_length_word(min_length: usize, max_length: usize, rng: &mut impl Rng) -> String {
    let target_length = if min_length == max_length {
        min_length
    } else {
        rng.random_range(min_length..=max_length)
    };

    if target_length == 3 {
        let common_three_letter_words = [
            "and", "the", "but", "for", "not", "you", "she", "his", "her", "who", "all", "one",
            "out", "use", "say", "has", "man", "day", "get", "see", "now", "may", "way", "new",
            "two", "any", "set", "boy", "put", "old", "too", "sea", "big", "eye", "high", "run",
            "cat", "dog", "car", "sun", "cup", "hat", "bed", "bus", "map", "cow", "pig", "red",
            "lot", "bad", "hot", "kid", "eat", "cut", "let", "top", "arm", "leg", "sit", "box",
            "egg", "key", "pen", "fan", "art", "web", "ice", "air", "gap", "wet", "job", "low",
            "few", "buy", "own", "try", "fun", "end", "act", "bit", "pay", "law", "oil", "mix",
            "raw", "fly", "far", "ask", "ago", "add",
        ];
        return common_three_letter_words[rng.random_range(0..common_three_letter_words.len())]
            .to_string();
    }

    let vowels = ["a", "e", "i", "o", "u"];
    let consonants = [
        "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "r", "s", "t", "v", "w", "y",
        "z",
    ];

    let mut word = String::new();

    if rng.random_bool(0.75) {
        word.push_str(consonants[rng.random_range(0..consonants.len())]);
    } else {
        word.push_str(vowels[rng.random_range(0..vowels.len())]);
    }

    while word.len() < target_length {
        if word.ends_with(|c: char| "aeiou".contains(c)) {
            word.push_str(consonants[rng.random_range(0..consonants.len())]);
        } else {
            word.push_str(vowels[rng.random_range(0..vowels.len())]);
        }
    }

    word.truncate(target_length);

    word
}

fn generate_pattern_based_word(min_length: usize, max_length: usize, rng: &mut impl Rng) -> String {
    let (initial_consonants, middle_consonants, final_consonants, vowels, endings) =
        get_phonetics();

    let target_length = rng.random_range(min_length..=max_length);

    let mut word = String::new();

    let syllable_count = match target_length {
        1..=3 => 1,
        4..=6 => rng.random_range(1..=2),
        7..=10 => rng.random_range(2..=3),
        _ => rng.random_range(2..=4),
    };

    let start_with_consonant = rng.random_bool(0.7);

    if start_with_consonant {
        word.push_str(initial_consonants[rng.random_range(0..initial_consonants.len())]);
    }

    let mut consecutive_vowels = 0;
    for i in 0..syllable_count {
        if word.len() > target_length.saturating_sub(3) {
            break;
        }

        if word.is_empty() || !ends_with_vowel(&word) {
            let vowel = if i == 0 {
                vowels[rng.random_range(0..vowels.len())]
            } else {
                ["a", "e", "i", "o", "u"][rng.random_range(0..5)]
            };

            word.push_str(vowel);
            consecutive_vowels += 1;
        }

        if word.len() < target_length.saturating_sub(3) && consecutive_vowels > 0 {
            let consonant = if i == 0 {
                middle_consonants[rng.random_range(0..middle_consonants.len())]
            } else {
                let simple_consonants = [
                    "b", "c", "d", "f", "g", "k", "l", "m", "n", "p", "r", "s", "t",
                ];
                simple_consonants[rng.random_range(0..simple_consonants.len())]
            };

            word.push_str(consonant);
            consecutive_vowels = 0;
        }
    }

    if !contains_vowel(&word) {
        let simple_vowel = ["a", "e", "i", "o", "u"][rng.random_range(0..5)];

        if word.len() >= 2 {
            let pos = word.len() / 2;
            word.insert_str(pos, simple_vowel);
        } else {
            word.push_str(simple_vowel);
        }
    }

    if target_length > 4 && word.len() <= target_length - 2 && rng.random_bool(0.6) {
        let suitable_endings: Vec<&str> = endings
            .iter()
            .filter(|&ending| word.len() + ending.len() <= target_length)
            .copied()
            .collect();

        if !suitable_endings.is_empty() {
            word.push_str(suitable_endings[rng.random_range(0..suitable_endings.len())]);
        }
    } else if target_length.saturating_sub(word.len()) > 0 {
        if ends_with_vowel(&word) && rng.random_bool(0.7) {
            word.push_str(final_consonants[rng.random_range(0..final_consonants.len())]);
        } else if !ends_with_vowel(&word) && rng.random_bool(0.3) {
            word.push_str(["a", "e", "i", "o", "u"][rng.random_range(0..5)]);
        }
    }

    if word.len() > target_length {
        let mut cut_index = target_length;

        while cut_index > target_length / 2 && !is_clean_break(&word, cut_index) {
            cut_index -= 1;
        }

        if cut_index < target_length / 2 {
            cut_index = target_length;
        }

        word.truncate(cut_index);
    }

    if word.len() < min_length {
        ensure_minimum_length(&mut word, min_length, rng);
    }

    fix_uncommon_patterns(&mut word);

    word
}

fn fix_uncommon_patterns(word: &mut String) {
    let uncommon_patterns = [
        "qm", "qn", "qr", "qw", "qf", "qp", "jx", "jz", "qz", "vq", "jq", "zq", "vz", "vj", "zx",
        "xz", "hx", "hq", "xq", "wq",
    ];

    for pattern in &uncommon_patterns {
        if word.contains(pattern) {
            let replacement = match pattern.chars().next().unwrap() {
                'q' => "cu",
                'j' => "g",
                'z' => "s",
                'v' => "f",
                'x' => "ks",
                'w' => "w",
                'h' => "h",
                _ => "e",
            };

            *word = word.replace(pattern, replacement);
        }
    }

    let difficult_clusters = [
        "bsd", "bsf", "bsg", "bsj", "bsk", "bsl", "bsm", "bsn", "bsp", "bsq", "bsr", "bst", "bsv",
        "bsw", "bsx", "bsy", "bsz", "czd", "czf", "czg", "czj", "czk", "czl", "czm", "czn", "czp",
        "czq", "czr", "czs", "czv", "czw", "czx", "czy", "jbd", "jbf", "jbg", "jbj", "jbk", "jbl",
        "jbm", "jbn", "jbp", "jbq", "jbr", "jbs", "jbt", "jbv", "jbw", "jbx", "jby", "jbz", "xbd",
        "xbf", "xbg", "xbj", "xbk", "xbl", "xbm", "xbn", "xbp", "xbq", "xbr", "xbs", "xbt", "xbv",
        "xbw", "xby", "xbz",
    ];

    for cluster in &difficult_clusters {
        if word.contains(cluster) {
            let mid = cluster.len() / 2;
            let replacement = cluster[0..mid].to_string() + "e" + &cluster[mid..];
            *word = word.replace(cluster, &replacement);
        }
    }
}

fn generate_common_english_word(
    min_length: usize,
    max_length: usize,
    rng: &mut impl Rng,
) -> String {
    let common_prefixes = [
        "re", "un", "in", "im", "dis", "en", "non", "com", "con", "pre", "pro", "ex", "sub",
        "inter", "trans", "super", "semi", "anti", "auto", "bi", "co", "de", "over", "under",
        "mis", "out",
    ];

    let common_roots = [
        "act", "art", "auth", "car", "cent", "cept", "cern", "cide", "cord", "curr", "dict", "doc",
        "duc", "fac", "fer", "form", "fort", "gen", "grad", "graph", "gress", "ject", "jud", "log",
        "luc", "man", "mand", "mark", "ment", "mer", "mit", "mov", "nat", "nect", "not", "nounce",
        "pend", "plic", "ply", "port", "pos", "press", "scrib", "sect", "sent", "sist", "spec",
        "struct", "tain", "tend", "tract", "vert", "vid", "vis", "voc", "word", "work", "bal",
        "ber", "bor", "cal", "can", "cel", "cen", "col", "dal", "der", "dev", "ech", "fin", "gal",
        "gar", "hap", "har", "hor", "jen", "kel", "ken", "lan", "lar", "len", "lin", "mar", "mel",
        "min", "nar", "nel", "ner", "pan", "par", "pen", "per", "ran", "ral", "ren", "sal", "san",
        "sar", "sel", "sen", "ser", "tan", "tel", "ter", "val", "van", "ver", "vin", "wel",
    ];

    let common_suffixes = [
        "able", "al", "ance", "ant", "ar", "ary", "ate", "ble", "dom", "ed", "en", "ence", "ent",
        "er", "est", "ful", "hood", "ian", "ible", "ic", "ical", "ice", "ify", "ile", "ing", "ion",
        "ious", "ish", "ism", "ist", "ite", "ity", "ive", "ize", "less", "ly", "ment", "ness",
        "or", "ory", "ous", "ship", "sion", "some", "tion", "ty", "ure", "ward", "wise", "y", "a",
        "o", "y", "ar", "er", "or", "ur", "us", "um", "ix", "ex", "ax",
    ];

    let target_length = rng.random_range(min_length..=max_length);

    let mut word = String::new();

    if target_length > 4 && rng.random_bool(0.4) {
        let prefix = common_prefixes[rng.random_range(0..common_prefixes.len())];
        if prefix.len() < target_length - 2 {
            word.push_str(prefix);
        }
    }

    let available_length = target_length - word.len();
    let suitable_roots: Vec<&str> = common_roots
        .iter()
        .filter(|&r| r.len() <= available_length)
        .copied()
        .collect();

    if !suitable_roots.is_empty() {
        let root = suitable_roots[rng.random_range(0..suitable_roots.len())];
        word.push_str(root);
    } else {
        let vowels = ["a", "e", "i", "o", "u"];
        let consonants = [
            "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "r", "s", "t", "v", "w",
        ];

        let v = vowels[rng.random_range(0..vowels.len())];
        let c = consonants[rng.random_range(0..consonants.len())];
        word.push_str(c);
        word.push_str(v);

        if available_length > 2 {
            word.push_str(consonants[rng.random_range(0..consonants.len())]);
        }
    }

    let available_length = target_length - word.len();
    if available_length > 0 {
        let suitable_suffixes: Vec<&str> = common_suffixes
            .iter()
            .filter(|&s| s.len() <= available_length)
            .copied()
            .collect();

        if !suitable_suffixes.is_empty() {
            let suffix = suitable_suffixes[rng.random_range(0..suitable_suffixes.len())];
            word.push_str(suffix);
        }
    }

    ensure_minimum_length(&mut word, min_length, rng);

    if word.len() > max_length {
        word.truncate(max_length);
    }

    word
}

fn generate_phoneme_based_word(min_length: usize, max_length: usize, rng: &mut impl Rng) -> String {
    let consonant_phonemes = vec![
        ("b", 20),
        ("ch", 15),
        ("d", 42),
        ("f", 22),
        ("g", 17),
        ("h", 34),
        ("j", 10),
        ("k", 45),
        ("l", 40),
        ("m", 26),
        ("n", 67),
        ("p", 19),
        ("r", 60),
        ("s", 63),
        ("sh", 16),
        ("t", 90),
        ("th", 33),
        ("v", 11),
        ("w", 15),
        ("y", 20),
        ("z", 7),
    ];

    let vowel_phonemes = vec![
        ("a", 80),
        ("e", 120),
        ("i", 70),
        ("o", 75),
        ("u", 27),
        ("ai", 20),
        ("ay", 15),
        ("ea", 25),
        ("ee", 20),
        ("oa", 10),
        ("oo", 15),
    ];

    let consonant_dist = to_vector_str(consonant_phonemes);

    let vowel_dist = to_vector_str(vowel_phonemes);

    let mut word = String::new();
    let target_length = rng.random_range(min_length..=max_length);

    let start_with_consonant = rng.random_bool(0.7);
    let mut is_consonant_turn = start_with_consonant;

    let mut consecutive_consonants = 0;
    let mut consecutive_vowels = 0;

    while word.len() < target_length {
        if consecutive_consonants >= 2 {
            is_consonant_turn = false;
            consecutive_consonants = 0;
        } else if consecutive_vowels >= 2 {
            is_consonant_turn = true;
            consecutive_vowels = 0;
        }

        let phoneme = if is_consonant_turn {
            consecutive_consonants += 1;
            consecutive_vowels = 0;

            if consecutive_consonants > 1 {
                ["b", "d", "f", "g", "k", "l", "m", "n", "p", "r", "s", "t"]
                    [rng.random_range(0..12)]
            } else {
                consonant_dist[rng.random_range(0..consonant_dist.len())]
            }
        } else {
            consecutive_vowels += 1;
            consecutive_consonants = 0;

            if consecutive_vowels > 1 {
                ["a", "e", "i", "o", "u"][rng.random_range(0..5)]
            } else {
                vowel_dist[rng.random_range(0..vowel_dist.len())]
            }
        };

        if word.len() + phoneme.len() <= target_length {
            word.push_str(phoneme);
            is_consonant_turn = !is_consonant_turn;
        } else {
            let fallback = if is_consonant_turn {
                [
                    "b", "d", "f", "g", "k", "l", "m", "n", "p", "s", "t", "v", "z",
                ][rng.random_range(0..13)]
            } else {
                ["a", "e", "i", "o", "u"][rng.random_range(0..5)]
            };

            if word.len() + fallback.len() <= target_length {
                word.push_str(fallback);
                is_consonant_turn = !is_consonant_turn;
            } else {
                break;
            }
        }
    }

    if word.len() >= 4 && word.len() + 2 <= max_length && rng.random_bool(0.3) {
        add_english_ending(&mut word, rng);
    }

    ensure_minimum_length(&mut word, min_length, rng);

    word
}

fn generate_syllable_based_word(
    min_length: usize,
    max_length: usize,
    rng: &mut impl Rng,
) -> String {
    let patterns = vec![
        ("CV", 40),
        ("CVC", 100),
        ("VC", 30),
        ("CCV", 20),
        ("CVCC", 25),
        ("CCVC", 20),
    ];

    let pattern_dist = to_vector_str(patterns);

    let mut word = String::new();
    let target_length = rng.random_range(min_length..=max_length);

    let syllable_count = match target_length {
        1..=3 => 1,
        4..=6 => rng.random_range(1..=2),
        7..=9 => rng.random_range(2..=3),
        _ => rng.random_range(2..=4),
    };

    for i in 0..syllable_count {
        let pattern = if i == 0 {
            pattern_dist[rng.random_range(0..pattern_dist.len())]
        } else {
            let simpler_patterns = vec![("CV", 60), ("CVC", 80), ("VC", 40)];
            let simpler_dist = to_vector_str(simpler_patterns);

            simpler_dist[rng.random_range(0..simpler_dist.len())]
        };

        let syllable = create_syllable(pattern, rng);

        if word.len() + syllable.len() <= target_length {
            word.push_str(&syllable);
        } else {
            let simple_pattern = if i == 0 { "CV" } else { "VC" };
            let simple_syllable = create_syllable(simple_pattern, rng);

            if word.len() + simple_syllable.len() <= target_length {
                word.push_str(&simple_syllable);
            }
        }
    }

    if word.len() >= 3 && word.len() + 2 <= max_length && rng.random_bool(0.3) {
        add_english_ending(&mut word, rng);
    }

    ensure_minimum_length(&mut word, min_length, rng);

    if word.len() > max_length {
        let mut cut_index = max_length;
        while cut_index > 0 && !is_clean_break(&word, cut_index) {
            cut_index -= 1;
        }

        if cut_index < max_length / 2 {
            cut_index = max_length;
        }

        word.truncate(cut_index);
    }

    word
}

fn create_syllable(pattern: &str, rng: &mut impl Rng) -> String {
    let mut syllable = String::new();

    for c in pattern.chars() {
        match c {
            'C' => {
                if syllable
                    .chars()
                    .last()
                    .map_or(false, |last| !"aeiou".contains(last))
                {
                    let single_consonants = [
                        "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "r", "s", "t",
                        "v", "w", "y", "z",
                    ];
                    syllable
                        .push_str(single_consonants[rng.random_range(0..single_consonants.len())]);
                } else {
                    let consonants = [
                        "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "r", "s", "t",
                        "v", "w", "z", "bl", "br", "ch", "cl", "cr", "dr", "fl", "fr", "gl", "gr",
                        "pl", "pr", "sh", "st", "th", "tr",
                    ];

                    if rng.random_bool(0.7) {
                        syllable.push_str(&consonants[0..18][rng.random_range(0..18)]);
                    } else {
                        syllable.push_str(&consonants[18..][rng.random_range(0..16)]);
                    }
                }
            }
            'V' => {
                let vowels = [
                    "a", "a", "e", "e", "e", "i", "i", "o", "o", "u", "ai", "ay", "ea", "ee", "oo",
                ];

                if rng.random_bool(0.85) {
                    syllable.push_str(&vowels[0..10][rng.random_range(0..10)]);
                } else {
                    syllable.push_str(&vowels[10..][rng.random_range(0..5)]);
                }
            }
            _ => {}
        }
    }

    syllable
}

fn score_english_word(word: &str) -> f64 {
    let letter_freq = [
        ('e', 0.12),
        ('t', 0.09),
        ('a', 0.08),
        ('o', 0.075),
        ('i', 0.07),
        ('n', 0.067),
        ('s', 0.063),
        ('r', 0.06),
        ('h', 0.06),
        ('l', 0.04),
        ('d', 0.042),
        ('c', 0.028),
        ('u', 0.028),
        ('m', 0.025),
        ('f', 0.022),
        ('p', 0.02),
        ('g', 0.02),
        ('w', 0.02),
        ('y', 0.02),
        ('b', 0.015),
        ('v', 0.01),
        ('k', 0.008),
        ('j', 0.002),
        ('x', 0.002),
        ('q', 0.001),
        ('z', 0.001),
    ]
    .iter()
    .copied()
    .collect::<HashMap<char, f64>>();

    let common_bigrams = [
        "th", "he", "in", "er", "an", "re", "on", "at", "en", "nd", "ti", "es", "or", "te", "of",
        "ed", "is", "it", "al", "ar", "st", "to", "nt", "ng", "se", "ha", "as", "ou", "io", "le",
        "ve", "co", "me", "de", "hi", "ri", "ro", "ic", "ne", "ea", "ra", "ce", "li", "ch", "ll",
        "be", "ma", "si", "om", "ur",
    ]
    .iter()
    .copied()
    .collect::<HashSet<&str>>();

    let common_trigrams = [
        "the", "and", "ing", "ion", "tio", "ent", "ati", "for", "her", "ter", "hat", "tha", "ere",
        "ate", "his", "con", "res", "ver", "all", "ons", "nce", "men", "ith", "ted", "ers", "pro",
        "thi", "wit", "are", "ess", "not", "ive", "was", "ect", "rea", "com", "eve", "per", "int",
        "est",
    ]
    .iter()
    .copied()
    .collect::<HashSet<&str>>();

    let uncommon_clusters = [
        "bx", "cj", "fq", "gx", "hx", "jz", "kq", "mx", "px", "qz", "vj", "vq", "vx", "wx", "xj",
        "zx", "zq", "qj", "qk", "qv", "qx", "qw", "jq", "jx", "bz", "cv", "dq", "fz", "gq", "hj",
        "kx", "kz", "mq", "mz", "pq", "tq", "vz", "wq", "wz", "xd",
    ]
    .iter()
    .copied()
    .collect::<HashSet<&str>>();

    let uncommon_vowel_sequences = [
        "aaa", "eee", "iii", "ooo", "uuu", "aei", "aeu", "aio", "aiu", "eio", "eiu", "iou", "uei",
        "uoi", "uou",
    ]
    .iter()
    .copied()
    .collect::<HashSet<&str>>();

    let bad_endings = [
        "q", "j", "v", "zl", "zm", "zn", "zr", "hj", "bx", "cx", "dx", "fx", "gx", "kx", "mx",
        "px", "vx", "wx", "xx", "zx",
    ]
    .iter()
    .copied()
    .collect::<HashSet<&str>>();

    let mut score = 0.0;

    for c in word.chars() {
        score += letter_freq.get(&c).copied().unwrap_or(0.0);
    }

    for i in 0..word.len().saturating_sub(1) {
        if i + 2 <= word.len() {
            let bigram = &word[i..i + 2];
            if common_bigrams.contains(bigram) {
                score += 0.3;
            }
        }
    }

    for i in 0..word.len().saturating_sub(2) {
        if i + 3 <= word.len() {
            let trigram = &word[i..i + 3];
            if common_trigrams.contains(trigram) {
                score += 0.5;
            }
        }
    }

    for i in 0..word.len().saturating_sub(1) {
        if i + 2 <= word.len() {
            let bigram = &word[i..i + 2];
            if uncommon_clusters.contains(bigram) {
                score -= 2.5;
            }
        }
    }

    let mut consonant_count = 0;
    for c in word.chars() {
        if "bcdfghjklmnpqrstvwxyz".contains(c) {
            consonant_count += 1;
            if consonant_count >= 3 {
                score -= 1.5;
                break;
            }
        } else {
            consonant_count = 0;
        }
    }

    for i in 0..word.len().saturating_sub(2) {
        if i + 3 <= word.len() {
            let vowel_seq = &word[i..i + 3];
            if uncommon_vowel_sequences.contains(vowel_seq) {
                score -= 2.5;
            }
        }
    }

    let mut vowel_count = 0;
    for c in word.chars() {
        if "aeiou".contains(c) {
            vowel_count += 1;
            if vowel_count > 2 {
                score -= 1.5;
                break;
            }
        } else {
            vowel_count = 0;
        }
    }

    let vowel_count = word.chars().filter(|&c| "aeiou".contains(c)).count() as f64;
    let ratio = vowel_count / word.len() as f64;

    if ratio < 0.25 || ratio > 0.55 {
        score -= 1.5;
    } else if ratio >= 0.38 && ratio <= 0.42 {
        score += 0.5;
    }

    let good_endings = [
        "ed", "ing", "er", "ly", "tion", "able", "ful", "ness", "ment", "ity", "s", "es", "al",
        "ive", "ate", "age", "ent", "ist", "ism", "ous", "a", "e", "o", "y", "n", "t", "r", "l",
        "d",
    ];
    for ending in &good_endings {
        if word.ends_with(ending) {
            score += 0.5;
            break;
        }
    }

    for ending in &bad_endings {
        if word.ends_with(ending) {
            score -= 1.5;
            break;
        }
    }

    let common_short_words = [
        "a", "an", "as", "at", "be", "by", "do", "go", "he", "hi", "if", "in", "is", "it", "me",
        "my", "no", "of", "on", "or", "so", "to", "up", "us", "we", "am", "are", "and", "but",
        "can", "did", "for", "get", "had", "has", "her", "him", "his", "how", "man", "new", "not",
        "now", "old", "one", "our", "out", "say", "see", "she", "the", "too", "who", "why", "you",
        "all", "any",
    ]
    .iter()
    .copied()
    .collect::<HashSet<&str>>();

    if common_short_words.contains(word) {
        score += 5.0;
    }

    if word.len() >= 4 && word.len() <= 8 {
        score += 0.5;
    }

    score
}

fn add_english_ending(word: &mut String, rng: &mut impl Rng) {
    if word.len() < 3 {
        return;
    }

    let endings = vec![
        ("", 300),
        ("ed", 40),
        ("ing", 40),
        ("ly", 25),
        ("er", 30),
        ("est", 10),
        ("ful", 10),
        ("ness", 10),
        ("ment", 10),
        ("ity", 10),
        ("ic", 8),
        ("al", 12),
        ("ous", 10),
        ("able", 8),
        ("ible", 5),
        ("en", 10),
        ("y", 25),
        ("s", 30),
        ("es", 15),
        ("a", 20),
        ("e", 20),
        ("o", 10),
    ];

    let ending_dist = to_vector_str(endings);

    let ending = ending_dist[rng.random_range(0..ending_dist.len())];

    if !ending.is_empty() && word.len() + ending.len() <= 15 {
        if ending == "ing" && word.ends_with('e') {
            word.truncate(word.len() - 1);
        } else if ending == "ed" && word.ends_with('e') {
            word.push('d');
            return;
        } else if (ending == "s" || ending == "es")
            && (word.ends_with('s')
                || word.ends_with('x')
                || word.ends_with('z')
                || word.ends_with("ch")
                || word.ends_with("sh"))
        {
            word.push_str("es");
            return;
        }

        word.push_str(ending);
    }
}

fn ensure_minimum_length(word: &mut String, min_length: usize, rng: &mut impl Rng) {
    while word.len() < min_length {
        if word.ends_with(|c: char| "aeiou".contains(c)) {
            let common_endings = ["n", "t", "r", "s", "l", "d", "m", "p"];
            word.push_str(common_endings[rng.random_range(0..common_endings.len())]);
        } else {
            let common_vowels = ["a", "e", "i", "o"];
            word.push_str(common_vowels[rng.random_range(0..common_vowels.len())]);
        }
    }
}
fn generate_short_english_word(length: usize, rng: &mut impl Rng) -> String {
    match length {
        1 => ["a", "a", "i", "o", "e"][rng.random_range(0..5)].to_string(),
        2 => {
            let options = [
                "an", "as", "at", "be", "by", "do", "go", "he", "hi", "if", "in", "is", "it", "me",
                "my", "no", "of", "on", "or", "so", "to", "up", "us", "we", "am", "an", "ba", "bo",
                "ca", "co", "da", "de", "di", "du", "fa", "fe", "fi", "fo", "ga", "ge", "ha", "ho",
                "ja", "jo", "ka", "ke", "ki", "la", "le", "li", "lo", "ma", "mi", "mo", "mu", "na",
                "ne", "ni", "nu", "pa", "pe", "pi", "po", "pu", "ra", "re", "ri", "ro", "ru", "sa",
                "se", "si", "su", "ta", "te", "ti", "to", "tu", "va", "ve", "vi", "vo", "wa", "we",
                "wi", "wo", "ya", "ye", "yo", "yu", "za", "ze", "zo",
            ];
            options[rng.random_range(0..options.len())].to_string()
        }
        3 => {
            let real_words = [
                "and", "art", "ask", "bad", "bag", "bar", "bat", "bed", "bee", "big", "bit", "box",
                "boy", "bug", "bus", "but", "buy", "can", "cap", "car", "cat", "cow", "cry", "cup",
                "cut", "dad", "day", "did", "dig", "dog", "dot", "dry", "due", "ear", "eat", "egg",
                "end", "eye", "fan", "far", "fat", "few", "fig", "fit", "fix", "fly", "for", "fox",
                "fun", "gap", "gas", "get", "got", "gum", "gun", "gym", "had", "ham", "has", "hat",
                "her", "hey", "him", "hip", "his", "hit", "hop", "hot", "how", "hub", "hug", "hut",
                "ice", "ill", "ink", "jar", "jaw", "jet", "job", "jog", "joy", "key", "kid", "kit",
                "lab", "lag", "lap", "law", "lay", "leg", "let", "lid", "lie", "lip", "lit", "log",
                "lot", "low", "mad", "man", "map", "mat", "may", "men", "met", "mix", "mom", "mop",
                "mud", "mug", "nap", "net", "new", "nil", "nod", "not", "now", "nut", "odd", "off",
                "oil", "old", "one", "our", "out", "owe", "own", "pad", "pan", "paw", "pay", "pen",
                "pet", "pie", "pig", "pin", "pit", "pop", "pot", "put", "rad", "rag", "ran", "rap",
                "rat", "raw", "red", "rib", "rid", "rim", "rip", "rob", "rod", "rot", "row", "rub",
                "rug", "run", "sad", "saw", "say", "sea", "see", "set", "she", "shy", "sin", "sip",
                "sir", "sit", "six", "ski", "sky", "sly", "son", "spy", "sum", "sun", "tab", "tag",
                "tan", "tap", "tar", "tax", "tea", "ten", "the", "tie", "tin", "tip", "toe", "ton",
                "too", "top", "toy", "try", "tub", "two", "use", "van", "vat", "vet", "via", "vow",
                "war", "was", "wax", "way", "web", "wed", "wet", "who", "why", "wig", "win", "wit",
                "won", "wow", "yes", "yet", "you", "zip", "zoo",
            ];

            if rng.random_bool(0.8) {
                real_words[rng.random_range(0..real_words.len())].to_string()
            } else {
                let patterns = ["CVC", "VCC", "CCV"];
                let pattern = patterns[rng.random_range(0..patterns.len())];
                create_syllable(pattern, rng)
            }
        }
        _ => "".to_string(),
    }
}

fn to_vector_str(array: Vec<(&str, usize)>) -> Vec<&str> {
    let dict = array
        .iter()
        .flat_map(|(p, w)| std::iter::repeat(*p).take(*w))
        .collect::<Vec<_>>();

    dict
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
