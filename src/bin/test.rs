#![feature(box_syntax)]

extern crate barrel;

use barrel::*;
use barrel::backend::{SqlGenerator, Pg};

fn main() {
    use Type::*;
    let mut m = Migration::new();

    let mut a;
    a = Pg::add_column("Text", Text);
    println!("\"Text\" ==> {}", a);

    a = Pg::add_column("Varchar", Varchar(255));
    println!("\"Varchar\" ==> {}", a);

    a = Pg::add_column("Integer", Integer);
    println!("\"Integer\" ==> {}", a);

    a = Pg::add_column("Float", Float);
    println!("\"Float\" ==> {}", a);

    a = Pg::add_column("Double", Double);
    println!("\"Double\" ==> {}", a);

    a = Pg::add_column("Boolean", Boolean);
    println!("\"Boolean\" ==> {}", a);

    a = Pg::add_column("Binary", Binary);
    println!("\"Binary\" ==> {}", a);

    a = Pg::add_column("Foreign", Foreign("posts"));
    println!("\"Foreign\" ==> {}", a);

    a = Pg::add_column("Array of Text", Array(box Text));
    println!("\"Array of Text\" ==> {}", a);

    a = Pg::add_column("Array of Varchar", Array(box Varchar(255)));
    println!("\"Array of Varchar\" ==> {}", a);

    a = Pg::add_column("Array of Integer", Array(box Integer));
    println!("\"Array of Integer\" ==> {}", a);

    a = Pg::add_column("Array of Float", Array(box Float));
    println!("\"Array of Float\" ==> {}", a);

    a = Pg::add_column("Array of Double", Array(box Double));
    println!("\"Array of Double\" ==> {}", a);

    a = Pg::add_column("Array of Boolean", Array(box Boolean));
    println!("\"Array of Boolean\" ==> {}", a);

    a = Pg::add_column("Array of Binary", Array(box Binary));
    println!("\"Array of Binary\" ==> {}", a);

    a = Pg::add_column("Array of Foreign", Array(box Foreign("posts")));
    println!("\"Array of Foreign\" ==> {}", a);

    a = Pg::add_column("Array of Array of Integer", Array(box Array(box Integer)));
    println!("\"Array of Integer\" ==> {}", a);




    // A new table is automatically created with an "id" primary key
    // To disable that call `without_id` on the return of `create_table`
    m.create_table("users", |t: &mut Table| {
        
        t.add_column("name", Varchar(255)).default("Anonymous"); // Default name is "Anonymous"
        t.add_column("description", Text).nullable(); // Can be null
        t.add_column("age", Integer);
        t.add_column("posts", Array(box Foreign("posts"))); // A one-to-many relationship
        t.add_column("owns_plushy_sharks", Boolean);
    });

    println!("{}", m.make::<Pg>());
}
