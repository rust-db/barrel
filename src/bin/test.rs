extern crate barrel;
use barrel::traits::*;
use barrel::*;


fn main() {

    let mut s = Schema::new().create_table("users", |t: &mut Table| {
        t.increments();
    });

}
