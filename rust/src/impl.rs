struct Releases {
    eigthies: i64,
    nineties: i64,
}

impl Releases {
    fn new(&self) -> Option<String> {
        if self.eigthies <= 0 || self.nineties <= 0 {
            return Some(format!("passe values greet than 0"));
        }
        return None;
    }
    fn get_nineties(&self) -> i64 {
        return self.nineties;
    }

    fn get_eigthies(&self) -> i64 {
        return self.eigthies;
    }
    fn change_eigthies_or_nineties(&mut self, eigthies: Option<i64>, nineties: Option<i64>) -> () {
        self.eigthies = match eigthies {
            Some(val) => { val }
            None => self.eigthies,
        };

        self.nineties = match nineties {
            Some(value) => { value }
            None => { self.nineties }
        };
    }

    fn get_some_nineties_and_eigthies(&self) -> i64 {
        return self.nineties + self.eigthies;
    }
}

fn main() {
    let mut releases: Releases = Releases { eigthies: 10, nineties: 20 };
    match releases.new() {
        Some(error) => panic!("{}", error),
        None => (),
    }
    println!("get_eigthies: {:?}", releases.get_eigthies());
    println!("get_nineties: {:?}", releases.get_nineties());
    println!("get_some_nineties_and_eigthies: {:?}", releases.get_some_nineties_and_eigthies());

    releases.change_eigthies_or_nineties(Some(200), Some(200));
    println!("get_eigthies: {:?}", releases.get_eigthies());
    println!("get_nineties: {:?}", releases.get_nineties());
}
