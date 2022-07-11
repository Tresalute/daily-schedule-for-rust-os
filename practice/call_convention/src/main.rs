use std::borrow::Borrow;


#[macro_use]
extern crate lazy_static;

lazy_static!{
    pub static ref STATICVAR:usize = static_init_func();
}

fn static_init_func() -> usize{
    1
}

fn main(){
    let x = *STATICVAR;
}