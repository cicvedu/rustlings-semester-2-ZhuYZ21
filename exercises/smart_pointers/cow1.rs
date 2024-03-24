use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        abs_all(&mut input);
        match input {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        abs_all(&mut input);
        match input {
            Cow::Borrowed(_) => Ok(()),
            _ => Err("Expected borrowed value"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        abs_all(&mut input);
        match input {
            Cow::Owned(_) => Err("Expected borrowed value"),
            Cow::Borrowed(_) => Ok(()),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        abs_all(&mut input);
        match input {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }
}
