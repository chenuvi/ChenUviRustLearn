mod lists;
mod temp;

mod lxqx;
#[allow(unused_imports)]
use lists::*;
#[allow(unused_imports)]
use lxqx::*;
fn main() {
    // iterator::run();
    // iterator_shoe
    // box_::run();

    // // query str
    // let query = String::from("name=Jack&age=18&hobby=game");
    // let qb = temp::request::QueryParser::from_string(&query);
    // println!("qb is {:#?}", qb);

    // thread::create_channel();
    // mutex_arc::run();

    // largest::run();

    multi_threads::run();
}
