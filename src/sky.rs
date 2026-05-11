use crate::star::Star;
use crate::{CONSTELLATION_SIZE, HEIGHT, NUM_OF_STARS, WIDTH};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct Sky(pub(crate) HashMap<String, Vec<Star>>);



impl Default for Sky {
    fn default() -> Self {
        let stars = generate_stars(NUM_OF_STARS);
        let constellations = group_by_distance(stars);
        Sky(constellations)
    }
}

impl Display for Sky {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let key = format!("{}:{}", x / CONSTELLATION_SIZE, y / CONSTELLATION_SIZE);

                // is there a constellation in this area
                match self.0.get(&key) {
                    Some(stars) => {
                        // if x % CONSTELLATION_SIZE == 0 && y % CONSTELLATION_SIZE == 0 {
                        //     str.push_str(&key);
                        // }

                        if stars.len() > 1 {
                            if x % CONSTELLATION_SIZE == 0 {
                                str.push('|');
                                continue;
                            }
                            if y % CONSTELLATION_SIZE == 0 {
                                str.push('_');
                                continue;
                            }
                        }

                        // is there a star in this pixel
                        match stars
                            .iter()
                            .find(|star| star.width == x && star.height == y)
                        {
                            Some(star) => str.push_str(&star.to_string()),
                            None => str.push(' '),
                        }
                    }
                    None => str.push(' '),
                };
            }
            str.push('\n');
        }
        write!(f, "{}", str)
    }
}


fn group_by_distance(stars: Vec<Star>) -> HashMap<String, Vec<Star>> {
    let mut grouped: HashMap<String, Vec<Star>> = HashMap::new();
    stars.into_iter().for_each(|star| {
        let key = star.get_constellation_key();
        grouped.entry(key).or_default().push(star);
    });
    grouped
}

fn generate_stars(count: usize) -> Vec<Star> {
    (0..count).map(|_| Star::generate_star()).collect()
}
