use std::collections::HashMap;

use noise::{core::perlin::perlin_2d, permutationtable::PermutationTable, utils::*};

/// Testing function that simply generates a new noise
/// then iterates over it and turns into a `HashMap`.
/// That `HashMap` uses `(usize, usize)` as a key and
/// `(f64, PointContext)` as a value
/// where `(usize, usize)` represents point on screen.
/// `PointContext` represents context of the point and `f64` its a point value
/// PS: Idk how to use this `f64` yet
///
/// Also returns `NoiseMap` for testing
fn noise() -> (HashMap<(usize, usize), (f64, PointContext)>, NoiseMap) {
    // generate new Perlin noise (or not Perlin?)
    let hasher = PermutationTable::new(15621548);
    let map = PlaneMapBuilder::new_fn(perlin_2d, &hasher)
        .set_size(10, 5)
        .set_x_bounds(-1.0, 1.0)
        .set_y_bounds(-1.0, 1.0)
        .build();

    map.write_to_file("15621548_50x40.png");

    let (_width, height) = map.size();

    let vec = map.iter().copied().collect::<Vec<_>>();
    let ch = vec
        .chunks(height)
        .scan(0, |w, c| {
            let out = c
                .iter()
                .copied()
                .map(|i| (i, PointContext::default()))
                .scan(0, |h, i| {
                    let out = ((*w, *h), i);
                    *h += 1;
                    Some(out)
                })
                .collect::<HashMap<_, _>>();
            *w += 1;
            Some(out)
        })
        .flatten()
        .collect::<HashMap<_, _>>();

    for i in &ch {
        println!("{:?}", i);
    }

    (ch, map)
}

/// This struct represents the context of current point
/// for example `allow_tree` defines ability of spawning trees
/// in this point
#[derive(Default, Debug)]
struct PointContext {
    allow_tree: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noise_test() {
        let (hashmap, map) = noise();
        assert_eq!(hashmap.get(&(0, 0)).unwrap().0, map.get_value(0, 0));
        assert_ne!(hashmap.get(&(5, 0)).unwrap().0, map.get_value(0, 0))
    }
}
