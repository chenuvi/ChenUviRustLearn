#[allow(dead_code)]
trait Park {
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: String) {
        println!("Painted {}", color);
    }
}

#[allow(dead_code)]
struct VehicleInfo {
    make: String,
    model: String,
    year: i32,
}
#[allow(dead_code)]
struct Car {
    info: VehicleInfo,
}
impl Park for Car {
    fn park(&self) {
        println!("Car parked");
    }
}
impl Paint for Car {}

#[allow(dead_code)]
struct Truck {
    info: VehicleInfo,
}
#[allow(dead_code)]
impl Truck {
    fn unload(&self) {
        println!("Truck unloaded");
    }
}
impl Park for Truck {
    fn park(&self) {
        println!("Truck parked");
    }
}
impl Paint for Truck {}

#[allow(dead_code)]
struct House {
    address: String,
    size: i32,
    model: String,
}
impl House {
    pub fn new(address: String, size: i32, model: String) -> Self {
        return House {
            address,
            size,
            model,
        };
    }
}
impl Paint for House {
    fn paint(&self, color: String) {
        println!("House painted {}", color);
    }
}

#[allow(dead_code)]
fn paint_red<T: Paint>(object: &T) {
    object.paint("red".to_owned());
}
// #[allow(dead_code)]
// fn paint_red2(object: impl Paint) {
//     object.paint("red".to_owned());
// }

// #[allow(dead_code)]
// fn paint_red3<T>(object: &T)
// where
//     T: Paint,
// {
//     object.paint("red".to_owned());
// }
fn paint_vihicle_red<T>(object: &T)
where
    T: Paint + Park,
{
    object.paint("red".to_owned());
}

fn create_paintable_object() -> impl Paint {
    House::new("123 Main St".to_owned(), 1000, "Modern".to_owned())
}
#[allow(dead_code)]
pub fn run() {
    let car = Car {
        info: VehicleInfo {
            make: "Honda".to_owned(),
            model: "Civic".to_owned(),
            year: 2020,
        },
    };

    let truck = Truck {
        info: VehicleInfo {
            make: "Ford".to_owned(),
            model: "F150".to_owned(),
            year: 2021,
        },
    };
    paint_red(&car);
    paint_red(&truck);
    let house = create_paintable_object();
    paint_red(&house);

    paint_vihicle_red(&car);
    paint_vihicle_red(&truck);
    // paint_vihicle_red(&house);
}
