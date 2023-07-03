enum CitySize {
    Town,       // approximate residents: 1_000
    City,       // approximate residents: 10_000
    Metropolis, // approximate residents: 1_000_000
}

struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

impl City {
    fn new(city_size: CitySize, is_coastal: bool) -> City {
        let (description, residents) = match city_size {
            CitySize::Town => {
                let residents = 1_000;

                (
                    format!("a *town* of approximately {} residents", residents),
                    residents,
                )
            }

            CitySize::Metropolis => {
                let residents: u64 = 1_000_000;

                (
                    format!("a *Metropolis* of approximately {} residents", residents),
                    residents,
                )
            }

            CitySize::City => {
                let residents: u64 = 100_000;

                (
                    format!("a *City* of approximantely {} residents", residents),
                    residents,
                )
            }
            _ => {
                let residents = 1_000;

                (
                    format!(
                        "an *unknown-size city* of approximately {} residents",
                        residents
                    ),
                    residents,
                )
            }
        };

        City {
            description,
            residents,
            is_coastal,
        }
    }
}

fn main() {
    let city: City = City::new(CitySize::City, true);
    let rustville: City = city;

    println!("This city is {}", rustville.description);
    println!("is_coastal {}", rustville.is_coastal);

    if rustville.residents > 100_000 {
        println!("Wow!");
    }
}
