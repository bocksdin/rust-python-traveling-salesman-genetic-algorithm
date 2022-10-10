



// UNNECESSARY DYNAMIC LENGTH VECTOR
let mut chromosome: Vec<i8> = vec![];
let mut rng: ThreadRng = rand::thread_rng();
for _ in 0..20 {
  chromosome.push(rng.gen_range(0..2));
}


// EFFICIENT FIXED LENGTH ARRAY
let mut chromosome: [i8; 20] = [0; 20];
let mut rng: ThreadRng = rand::thread_rng();
for i in 0..20 {
  chromosome[i] = rng.gen_range(0..2);
}




// UNNECESSARY COPYING OF ARRAY
pub fn fitness(&mut self) {
  // DO SOMETHING
  self.paths
  self.cities
}


// EFFICIENT REFERENCE OF ARRAY
pub fn fitness(&mut self, paths: &[i8; 20], cities: &[i8; 5]) {
  // DO SOMETHING
}




// LESS EFFICIENT ARRAY USE
pub fn crossover(&self, other_traveler: &Traveler, paths: &[Path; 20]) -> [Traveler; 2] {
  // ...
  [children1, children2]
}


// MORE EFFICIENT TUPLE USE
pub fn crossover(&self, other_traveler: &Traveler, paths: &[Path; 20]) -> (Traveler, Traveler) {
  // ...
  (children1, children2)
}




// MORE EXPENSIVE FUNCTION
pub fn fitness(&mut self, cities: &[i8; 5], paths: &[Path; 20]) {
  // EXPENSIVE CALCULATION
}


// LESS EXPENSIVE FUNCTION
pub fn fitness(&mut self, cities: &[i8; 5], paths: &[Path; 20]) {
  if self.chromosome.iter().filter(|x| x == &&(1 as i8)).count() != cities.len() {
      self.cost = f32::MAX;
      return;
  }

  // EXPENSIVE CALCULATION
}




