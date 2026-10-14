use rand::RngExt;

pub fn random_between(start: i16, end: i16) -> i16 {
    let min = start.min(end);
    let max = start.max(end);
    let mut rng = rand::rng();
    rng.random_range(min..=max)
}