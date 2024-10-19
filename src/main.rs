const VERSION: &str = "1.0.3";
const ALPHABET: [(char, i8, i8, i8); 26] = [
    ('a', 1, -25, 27),
    ('b', 2, -24, 28),
    ('c', 3, -23, 29),
    ('d', 4, -22, 30),
    ('e', 5, -21, 31),
    ('f', 6, -20, 32),
    ('g', 7, -19, 33),
    ('h', 8, -18, 34),
    ('i', 9, -17, 35),
    ('j', 10, -16, 36),
    ('k', 11, -15, 37),
    ('l', 12, -14, 38),
    ('m', 13, -13, 39),
    ('n', 14, -12, 40),
    ('o', 15, -11, 41),
    ('p', 16, -10, 42),
    ('q', 17, -9, 43),
    ('r', 18, -8, 44),
    ('s', 19, -7, 45),
    ('t', 20, -6, 46),
    ('u', 21, -5, 47),
    ('v', 22, -4, 48),
    ('w', 23, -3, 49),
    ('x', 24, -2, 50),
    ('y', 25, -1, 51),
    ('z', 26, 0, 52),
];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut string_list: Vec<String> = vec!();
    let mut key_start: usize  = 0;
    if args.len() > 2{
        string_list = convert_string(&args[3]);
    }

    if args.len() == 1 {
        println!("Invalid number of arguments");
    }else {
        match args[1].as_str() {
            "-h" | "--help" => help(),
            "-v" => println!("Quindro-Crypt v{}", VERSION),
            "-e" => {
                if args.len() != 4 {
                    eprintln!("Invalid number of arguments");
                } else {
                    for i in 0..string_list.len() {
                        encrypt(&args[2].to_lowercase(), &string_list[i].to_lowercase(), &mut key_start);
                    }
                    println!();
                }
            },
            "-d" => {
                if args.len() != 4 {
                    eprintln!("Invalid number of arguments");
                } else {


                    for i in 0..string_list.len() {
                        decrypt(&args[2].to_lowercase(), &string_list[i].to_lowercase(), &mut key_start);
                    }
                    println!();
                }
            },
            _ => eprintln!("Invalid option"),
        }
    }

}

fn convert_string(string: &str) -> Vec<String> {
    let mut string_to_list = string.to_lowercase().chars().collect::<Vec<char>>();
    string_to_list.push(' ');

    let mut string_list: Vec<String> = vec!();
    let mut last_num: usize = 0;
    let mut string_num = 0;
    for i in 0..string_to_list.len() {
        if string_to_list[i] == ' ' {
            string_list.push(String::new());
            for j in 0..i-last_num {
                string_list[string_num] += &string_to_list[last_num+j].to_string();

            }
            last_num = i + 1;
            string_num += 1;
        }
    }
    string_list
}

fn help() {
    println!("$ quindro-crypt \x1b[32m[-option]\x1b[0m

`\x1b[32m-h\x1b[0m` Shows the help message (also shown in 'README.md')

`\x1b[32m-v\x1b[0m` Shows the installed version

`\x1b[32m-e\x1b[0m \x1b[33m<key> <string>\x1b[0m` Encrypts the given string

`\x1b[32m-d\x1b[0m \x1b[33m<key> <string>\x1b[0m` Decrypts the given string");
}

fn generate_key(in_key: &str) -> (Vec<i8>,String) {
    let in_key_vec: Vec<char> = in_key.trim().chars().collect::<Vec<char>>();
    let mut key_vec: Vec<char> = vec!();
    let mut key_double_char = ' ';

    for i in 0..in_key_vec.len() {
        if !key_vec.contains(&in_key_vec[i]) {
            key_vec.push(in_key_vec[i]);
        }else if key_double_char == ' '{
            key_double_char = in_key_vec[i];
        }
    }
    if key_double_char == ' ' {
        key_double_char = in_key_vec[0];
    }

    if key_vec.len() < 4 {
        return (vec![1,2,3,4],"Key is to short | Must have at least 4 different letters".to_string())
    }

    let mut key: Vec<char> = vec!();
    for i in 0..4 {
        key.push(key_vec[i]);
    }
    key.push(key_double_char);

    let mut num_key: Vec<i8> = vec!();

    for i in 0..key.len() {
        for j in 0..ALPHABET.len() {
            if ALPHABET[j].0 == key[i] {
                num_key.push(ALPHABET[j].1);
            }
        }
    }

    (num_key," ".to_string())
}

fn encrypt(in_key: &str, in_string: &str, key_start: &mut usize) -> (){
    let key = generate_key(in_key);
    let key_start_copy = key_start.to_owned();
    if key.1.len() > 5 {
        eprintln!("{:?}",key.1);
    }

    let mut num_string: Vec<i8> = vec!();
    let string = in_string.to_lowercase().chars().collect::<Vec<char>>();

    for i in 0..string.len() {
        for j in 0..ALPHABET.len() {
            if ALPHABET[j].0 == string[i] {
                num_string.push(ALPHABET[j].1);
            }
        }
    }

    let mut out_num_string:Vec<i8> = vec!();
    for i in 0..num_string.len() {
        out_num_string.push(num_string[i] - key.0[(i + key_start_copy)% 5]);
        *key_start = (i + key_start_copy + 1)% 5;
    }

    let mut out_string = String::new();
    for i in 0..out_num_string.len() {
        for j in 0..ALPHABET.len() {
            if ALPHABET[j].1 == out_num_string[i] || ALPHABET[j].2 == out_num_string[i] {
                out_string.push(ALPHABET[j].0);
            }
        }
    }

    print!("{} ",out_string)
}

fn decrypt(in_key: &str, in_string: &str, key_start: &mut usize) -> () {
    let key = generate_key(in_key);
    let key_start_copy = key_start.to_owned();
    if key.1.len() > 5 {
        eprintln!("{:?}",key.1);
    }

    let mut num_string: Vec<i8> = vec!();
    let string = in_string.to_lowercase().chars().collect::<Vec<char>>();

    for i in 0..string.len() {
        for j in 0..ALPHABET.len() {
            if ALPHABET[j].0 == string[i] {
                num_string.push(ALPHABET[j].1);
            }
        }
    }

    let mut out_num_string:Vec<i8> = vec!();
    for i in 0..num_string.len() {
        out_num_string.push(num_string[i] + key.0[(i + key_start_copy.to_owned()) % 5]);
        *key_start = (i + key_start_copy + 1)% 5;
    }

    let mut out_string = String::new();
    for i in 0..out_num_string.len() {
        for j in 0..ALPHABET.len() {
            if ALPHABET[j].1 == out_num_string[i] || ALPHABET[j].2 == out_num_string[i] || ALPHABET[j].3 == out_num_string[i] {
                out_string.push(ALPHABET[j].0);
            }
        }
    }

    print!("{} ",out_string)
}