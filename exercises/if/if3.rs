pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = match animal {
        "crab" => 1,
        "gopher" => 2,
        "snake" => 3,
        _ => 0 // 使用0作为默认值，它不会匹配其他的分支
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = match identifier {
        1 => "Beach",
        2 => "Burrow",
        3 => "Desert",
        _ => "Unknown" // 匹配所有其他值，包括默认值0
    };

    habitat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
