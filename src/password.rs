use passwords::PasswordGenerator;

pub fn create_password(len: i32, num: bool, uppercase: bool, symbol: bool) -> String {
    let pg = PasswordGenerator {
        length: len as usize,
        numbers: num,
        lowercase_letters: true,
        uppercase_letters: uppercase,
        symbols: symbol,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };

    let password = pg.generate_one().unwrap();

    password
}