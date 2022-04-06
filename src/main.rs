mod lib;
mod models;

fn main() {
    println!("enter main!");
    let backend = sea_orm::DatabaseBackend::MySql;
    entity_center::get_list().iter().for_each(|e| {
        e(backend);
    });
    lib::invoke2("Alice","Bob");
    fn a () {

    }
    a();
    println!("Hello, world!");
}


