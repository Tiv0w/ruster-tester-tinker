// useless but cool to learn structs
struct Necklace {
    elements: String,
}

impl Necklace {
    fn new(elements: String) -> Necklace {
        Necklace { elements }
    }

    fn rotate_one(&mut self) {
        let rest = &self.elements[1..];
        let first = &self.elements[..1];
        self.elements = [rest, first].concat();
    }

    fn same(&mut self, other_necklace: String) -> bool {
        let elements_len = self.elements.len();
        if elements_len != other_necklace.len() {
            return false;
        }

        for _ in 1..elements_len {
            if self.elements == other_necklace {
                return true;
            }
            self.rotate_one();
        }

        false
    }
}
//

pub fn same_necklace() {
    let mut necklace = Necklace::new(String::from("salut"));
    println!("Same? {}", necklace.same(String::from("utsal")));

    let s1 = String::from("utsal");
    let s2 = String::from("salut");
    println!("Same? {}", same_necklaces(s1, s2));
}

// easy and fast implementation
pub fn same_necklaces(mut s1: String, s2: String) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let s3 = s1.clone();
    s1.push_str(&s3[..]);
    return s1.contains(&s2[..]);
}
