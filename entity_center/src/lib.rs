use std::sync::Mutex;
use sea_orm::DbBackend;

lazy_static::lazy_static! {
    static ref entity_list: Mutex<Vec<fn (DbBackend) -> ()>> = Mutex::new(Vec::new());
}

pub fn register(func: fn (DbBackend) -> ()) {
    entity_list.lock().unwrap().push(func);
}

pub fn get_list() -> Vec<fn (DbBackend) -> ()> {
    let newVec: Vec<fn (DbBackend) -> ()> = entity_list.lock().unwrap().clone();
    newVec
}