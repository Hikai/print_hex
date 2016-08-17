use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn alphabet_match_upper (hex: u8) -> String {
    let alphabet = match hex {
        10 => "A".to_string(),
        11 => "B".to_string(),
        12 => "C".to_string(),
        13 => "D".to_string(),
        14 => "E".to_string(),
        15 => "F".to_string(),
        _ => hex.to_string(),
    };

    return alphabet;
}

fn file_read(ref_path: &OsStr) -> [u8; 1024] {
    let mut byte_store = [0u8; 1024];

    let path = Path::new(ref_path);
    let mut file = File::open(&path).unwrap();
    file.read(&mut byte_store).unwrap();

    return byte_store;
}

fn ref_str_to_ref_osstr(arg_filename: &str) -> &OsStr {
    let ref_osstr: &OsStr = OsStr::new(arg_filename);

    return ref_osstr;
}

fn to_hex (chr: u8) -> String {
    let mut hex_operand = chr;
    let mut vec_hex = Vec::new();
    let mut str_result = String::new();

    loop {
        vec_hex.push(alphabet_match_upper(hex_operand % 16));
        hex_operand /= 16;

        if hex_operand > 15 {
            continue;
        }
        else {
            vec_hex.push(alphabet_match_upper(hex_operand));
            break;
        }
    }

    vec_hex.reverse();
    
    for item in vec_hex {
        str_result.push_str(item.as_ref());
    }
    
    return str_result;
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: binary_open.exe [filename]");
        std::process::exit(1);
    }

    let ref_osstr: &OsStr = ref_str_to_ref_osstr(&args[1]);

    let store = file_read(ref_osstr);

    let mut count = 0;
    for byte in store.iter() {
        print!("{:3} ", to_hex(*byte));
        if count >= 7 {
            println!("");

            count = 0;
        }
        else {
            count += 1;
        }
    }
}
