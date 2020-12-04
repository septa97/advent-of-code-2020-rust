use regex::Regex;
use std::fs;

struct Passport {
    byr_opt: Option<String>,
    iyr_opt: Option<String>,
    eyr_opt: Option<String>,
    hgt_opt: Option<String>,
    hcl_opt: Option<String>,
    ecl_opt: Option<String>,
    pid_opt: Option<String>,
}

fn main() {
    let file_string = fs::read_to_string("input/input.txt").expect("Failed to read file!");

    let byr_re = Regex::new(r"byr:\S+").unwrap();
    let iyr_re = Regex::new(r"iyr:\S+").unwrap();
    let eyr_re = Regex::new(r"eyr:\S+").unwrap();
    let hgt_re = Regex::new(r"hgt:\S+").unwrap();
    let hcl_re = Regex::new(r"hcl:\S+").unwrap();
    let ecl_re = Regex::new(r"ecl:\S+").unwrap();
    let pid_re = Regex::new(r"pid:\S+").unwrap();

    let valid_byr_re = Regex::new(r"^byr:\d{4}$").unwrap();
    let valid_iyr_re = Regex::new(r"^iyr:\d{4}$").unwrap();
    let valid_eyr_re = Regex::new(r"^eyr:\d{4}$").unwrap();
    let valid_hgt_re = Regex::new(r"^hgt:\d{2,3}(cm|in)$").unwrap();
    let valid_hcl_re = Regex::new(r"^hcl:#[0-9a-f]{6}$").unwrap();
    let vaild_ecl_re = Regex::new(r"^ecl:(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let valid_pid_re = Regex::new(r"^pid:\d{9}$").unwrap();

    let passports: Vec<Passport> = file_string
        .split("\n\n")
        .map(|split| {
            let byr_opt = byr_re.find(split).map(|s| s.as_str().to_string());
            let iyr_opt = iyr_re.find(split).map(|s| s.as_str().to_string());
            let eyr_opt = eyr_re.find(split).map(|s| s.as_str().to_string());
            let hgt_opt = hgt_re.find(split).map(|s| s.as_str().to_string());
            let hcl_opt = hcl_re.find(split).map(|s| s.as_str().to_string());
            let ecl_opt = ecl_re.find(split).map(|s| s.as_str().to_string());
            let pid_opt = pid_re.find(split).map(|s| s.as_str().to_string());

            Passport {
                byr_opt,
                iyr_opt,
                eyr_opt,
                hgt_opt,
                hcl_opt,
                ecl_opt,
                pid_opt,
            }
        })
        .collect();

    let part1_valid_count = passports
        .iter()
        .filter(|&passport| {
            let has_byr = passport.byr_opt.is_some();
            let has_iyr = passport.iyr_opt.is_some();
            let has_eyr = passport.eyr_opt.is_some();
            let has_hgt = passport.hgt_opt.is_some();
            let has_hcl = passport.hcl_opt.is_some();
            let has_ecl = passport.ecl_opt.is_some();
            let has_pid = passport.pid_opt.is_some();

            has_byr && has_iyr && has_eyr && has_hgt && has_hcl && has_ecl && has_pid
        })
        .count();

    let part2_valid_count = passports
        .iter()
        .filter(|&passport| {
            is_valid_byr(passport, &valid_byr_re)
                && is_valid_iyr(passport, &valid_iyr_re)
                && is_valid_eyr(passport, &valid_eyr_re)
                && is_valid_hgt(passport, &valid_hgt_re)
                && is_valid_hcl(passport, &valid_hcl_re)
                && is_valid_ecl(passport, &vaild_ecl_re)
                && is_valid_pid(passport, &valid_pid_re)
        })
        .count();

    println!("part 1: {}", part1_valid_count);
    println!("part 2: {}", part2_valid_count);
}

fn is_valid_byr(passport: &Passport, regex: &Regex) -> bool {
    passport
        .byr_opt
        .as_ref()
        .filter(|byr| regex.is_match(&byr[..]))
        .map(|byr| {
            let value = &byr[4..];
            let date: u32 = value.parse().expect("Failed to parse to u32");

            date >= 1920 && date <= 2002
        })
        .unwrap_or_else(|| false)
}

fn is_valid_iyr(passport: &Passport, regex: &Regex) -> bool {
    passport
        .iyr_opt
        .as_ref()
        .filter(|iyr| regex.is_match(&iyr[..]))
        .map(|iyr| {
            let value = &iyr[4..];
            let date: u32 = value.parse().expect("Failed to parse to u32");

            date >= 2010 && date <= 2020
        })
        .unwrap_or_else(|| false)
}

fn is_valid_eyr(passport: &Passport, regex: &Regex) -> bool {
    passport
        .eyr_opt
        .as_ref()
        .filter(|eyr| regex.is_match(&eyr[..]))
        .map(|eyr| {
            let value = &eyr[4..];
            let date: u32 = value.parse().expect("Failed to parse to u32");

            date >= 2020 && date <= 2030
        })
        .unwrap_or_else(|| false)
}

fn is_valid_hgt(passport: &Passport, regex: &Regex) -> bool {
    passport
        .hgt_opt
        .as_ref()
        .filter(|hgt| regex.is_match(&hgt[..]))
        .map(|hgt| {
            let value = &hgt[4..(hgt.len() - 2)];
            let unit = &hgt[(hgt.len() - 2)..];

            let height: u32 = value.parse().expect("Failed to parse to u32");

            if unit == "cm" {
                height >= 150 && height <= 193
            } else {
                // safe to assume that the other one is "in" because of VALIG_HGT_RE regex filter
                height >= 59 && height <= 76
            }
        })
        .unwrap_or_else(|| false)
}

fn is_valid_hcl(passport: &Passport, regex: &Regex) -> bool {
    passport
        .hcl_opt
        .as_ref()
        .map(|hcl| regex.is_match(&hcl[..]))
        .unwrap_or_else(|| false)
}

fn is_valid_ecl(passport: &Passport, regex: &Regex) -> bool {
    passport
        .ecl_opt
        .as_ref()
        .map(|ecl| regex.is_match(&ecl[..]))
        .unwrap_or_else(|| false)
}

fn is_valid_pid(passport: &Passport, regex: &Regex) -> bool {
    passport
        .pid_opt
        .as_ref()
        .map(|pid| regex.is_match(&pid[..]))
        .unwrap_or_else(|| false)
}
