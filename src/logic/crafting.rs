pub trait Item {
    fn ways(&self) -> Box<[&'static dyn Item]>;

    fn source(&self) -> Box<[&'static dyn Item]>;
}

// pub struct Rock;

// impl Item for Rock {
//     fn ways(&self) -> Box<[&'static dyn Item]> {
//         Box::new([&Stick])
//     }
// }

// pub struct Stick;

// impl Item for Stick {
//     fn ways(&self) -> Box<[&'static dyn Item]> {
//         Box::new([])
//     }
// }
