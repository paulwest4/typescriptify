#[macro_use]
extern crate typescriptify_derive;

use std::collections::HashSet;
use std::collections::HashMap;

trait TypeScriptifyTrait {
    fn type_script_ify() -> String;
}

#[derive(TypeScriptify)]
struct FrenchToast {
    pub i : u32,
    pub v : Vec<u8>,
    pub hashmap: HashMap<String, u16>,
    pub hashset: HashSet<u32>,
    pub optional: Option<bool>,
}

#[derive(TypeScriptify)]
struct Waffles {
    pub t : i64,
    pub x : bool,
    pub subtoast : FrenchToast,
}

#[derive(TypeScriptify)]
pub enum Sweet {
    Caroline {
        x : i64,
        b : bool,
        hashmap: HashMap<String, u16>,
        hashset: HashSet<u32>,
    },
    Sugar {
        i : u32,
        optional: Option<bool>,
        v : Vec<u8>,
    }
}

fn main() {
    //FrenchToast::hello_world();
    //Waffles::hello_world();
    println!("Typescript output for Waffles: \n{}", Waffles::type_script_ify() );
    println!("Typescript output for FrenchToast: \n{}", FrenchToast::type_script_ify() );
    println!("Typescript output for Sweet: \n{}", Sweet::type_script_ify() );
    //println!("Typescript type for Waffles: \n{}", Waffles::as_typescript_type() );
}