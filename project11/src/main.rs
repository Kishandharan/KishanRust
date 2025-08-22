const ugc : f64 = 0.000000000066743;

struct Sphere{
    mass: f64,
    radius: f64
}

impl Sphere{
    fn get_gravity(&self) -> f64{
        return (ugc*self.mass) / (self.radius*self.radius);
    }
}

fn main() {
    let sphere1 = Sphere{
        mass: 1000000 as f64,
        radius: 3 as f64
    };
    println!("{}", sphere1.get_gravity());
}
