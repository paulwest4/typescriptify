#[macro_use]
extern crate typescriptify_derive;


pub trait TypeScriptifyTrait {
    fn type_script_ify() -> String;
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::collections::HashMap;
    use TypeScriptifyTrait;

    #[derive(TypeScriptify)]
    struct VecTest {
        pub arr: Vec<i32>,
    }

    #[derive(TypeScriptify)]
    struct OptionTest {
        pub option: Option<i32>,
    }

    #[derive(TypeScriptify)]
    struct OptionVecTest {
        pub option: Option<Vec<i32>>,
    }

//    #[derive(TypeScriptify)]
//    struct FrenchToast {
//        pub i: u32,
//        pub v: Vec<u8>,
//        pub hashmap: HashMap<String, u16>,
//        pub hashset: HashSet<u32>,
//        pub optional: Option<bool>,
//    }
//
//    #[derive(TypeScriptify)]
//    struct Waffles {
//        pub t: i64,
//        pub s: usize,
//        pub x: bool,
//        pub subtoast: FrenchToast,
//    }
//
//    #[derive(TypeScriptify)]
//    pub enum Sweet {
//        Caroline {
//            x: i64,
//            b: bool,
//            hashmap: HashMap<String, u16>,
//            hashset: HashSet<u32>,
//        },
//        Sugar {
//            i: u32,
//            x: u64,
//            s: usize,
//            optional: Option<bool>,
//            v: Vec<u8>,
//        },
//    }
//
//
//    #[derive(TypeScriptify)]
//    pub enum Enum {
//        Created,
//        Finalized,
//        ExportedAtLeastOnce,
//    }


//    #[test]
//    fn test_works() {
//
//        let x = format!("Typescript output for Waffles: \n{}", Waffles::type_script_ify());
//        let y = format!("Typescript output for FrenchToast: \n{}", FrenchToast::type_script_ify());
//        let z = format!("Typescript output for Sweet: \n{}", Sweet::type_script_ify());
//
////        println!("Typescript outputs:\n{}\n{}\n{}\n", x, y, z);
//
//        assert_eq!(x.contains("subtoast: FrenchToast"), true);
//        assert_eq!(y.contains("hashmap: Map<string, number>"), true);
//        assert_eq!(z.contains("export interface Caroline"), true);
//    }
//
//    #[test]
//    fn test_enum() {
//        let r = format!("Typescript output for Enum: \n{}", Enum::type_script_ify());
////        println!("{}\n", r);
//        assert!(true);
//    }

    #[test(test_vec)]
    fn test_vec() {
        let r = format!("Typescript output for Array: \n{}", VecTest::type_script_ify());
        println!("{}\n", r);
        assert!(true);
    }

    #[test(test_option)]
    fn test_option() {
        let r = format!("Typescript output for Option: \n{}", OptionTest::type_script_ify());
        println!("{}\n", r);
        assert!(true);
    }

    #[test(test_option_vec)]
    fn test_option_vec() {
        let r = format!("Typescript output for Option: \n{}", OptionVecTest::type_script_ify());
        println!("{}\n", r);
        assert!(true);
    }
}