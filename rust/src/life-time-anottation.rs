struct Releases<'lifetime> {
    year: &'lifetime [i64],
    eigthies: &'lifetime [i64],
    nineties: &'lifetime [i64],
}

fn main() {
    let year: Vec<i64> = vec![1880, 1885, 1900, 1995, 2000, 2005];
    let jazz = jazz_realese(&year);
    println!(
        "eigthies: {:?}, nineties: {:?}, year: {:?}",
        jazz.eigthies, jazz.nineties, jazz.year
    );
}

fn jazz_realese<'lifetime>(year: &'lifetime [i64]) -> Releases {
    let eigthies: &'lifetime [i64] = &year[0..2];
    let nineties: &'lifetime [i64] = &year[2..4];
    Releases {
        year,
        eigthies,
        nineties,
    }
}
