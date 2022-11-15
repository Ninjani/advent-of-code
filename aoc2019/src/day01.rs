use utility::AocDay;

pub struct Day01;

impl<'a> AocDay<'a> for Day01 {
    type Input = Vec<f64>;
    type Result1 = f64;
    type Result2 = f64;

    fn day() -> usize {
        1
    }
    fn year() -> usize {
        2019
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(input
            .split('\n')
            .map(|line| line.parse::<f64>())
            .collect::<Result<_, _>>()?)
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        Ok(input.into_iter().map(|f| get_fuel_for_mass(*f)).sum())
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        Ok(input
            .into_iter()
            .map(|f| get_fuel_for_mass_and_fuel_r(*f))
            .sum())
    }
}

fn get_fuel_for_mass(mass: f64) -> f64 {
    (mass / 3.).floor() - 2.
}

/// iterative version
fn get_fuel_for_mass_and_fuel_i(mass: f64) -> f64 {
    let mut new_mass = mass;
    let mut fuel = 0.;
    while new_mass > 0. {
        new_mass = get_fuel_for_mass(new_mass);
        if new_mass > 0. {
            fuel += new_mass;
        } else {
            break;
        }
    }
    fuel
}

/// recursive version
fn get_fuel_for_mass_and_fuel_r(mass: f64) -> f64 {
    let new_mass = get_fuel_for_mass(mass);
    if new_mass > 0. {
        new_mass + get_fuel_for_mass_and_fuel_r(new_mass)
    } else {
        0.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2., get_fuel_for_mass(12.));
        assert_eq!(2., get_fuel_for_mass(14.));
        assert_eq!(654., get_fuel_for_mass(1969.));
        assert_eq!(33583., get_fuel_for_mass(100756.));
    }

    #[test]
    fn test_2_i() {
        assert_eq!(2., get_fuel_for_mass_and_fuel_i(14.));
        assert_eq!(966., get_fuel_for_mass_and_fuel_i(1969.));
        assert_eq!(50346., get_fuel_for_mass_and_fuel_i(100756.));
    }

    #[test]
    fn test_2_r() {
        assert_eq!(2., get_fuel_for_mass_and_fuel_r(14.));
        assert_eq!(966., get_fuel_for_mass_and_fuel_r(1969.));
        assert_eq!(50346., get_fuel_for_mass_and_fuel_r(100756.));
    }
}
