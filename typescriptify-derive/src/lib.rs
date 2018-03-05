extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(TypeScriptify)]
pub fn typescriptify_derive(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_typescriptify(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn typescriptify_rust_type<T: AsRef<str>>(rust_type: T) -> String {
    match rust_type.as_ref() {
        "i8" => "number",
        "i16" => "number",
        "i32" => "number",
        "i64" => "number",
        "u8" => "number",
        "u16" => "number",
        "u32" => "number",
        "u64" => "number",
        "usize" => "number",
        "bool" => "boolean",
        "String" => "string",
        "f32" => "number",
        "f64" => "number",
        "HashMap" => "Map",
        "Vec" => "Array",
        "HashSet" => "Array",
        "Value" => "any",
        a @ _ => a,
    }.to_string()
}

fn impl_typescriptify(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let structname = name.to_string();
    let _body = &ast.body;
    let complete_string: String = match ast.body {
        syn::Body::Struct(ref data) =>
            typescriptify_struct(data, &structname),

        syn::Body::Enum(ref variant_vec) =>
            typescriptify_enum(variant_vec, &structname),
    };
    quote! {
        impl TypeScriptifyTrait for #name {
            fn type_script_ify() -> String {
                format!("{}\n", #complete_string)
            }
        }
    }
}

fn typescriptify_struct(data: &syn::VariantData, structname: &str) -> String {
    let mut fieldlines: Vec<String> = vec![];
    for field in data.fields() {
        //field.ty; //type of the field
        //field.ident; //name (tuple struct fields have none here)
        //field.vis; //visibility
        //field.attrs; //attributes
        println!("Fieldtype = {:?} and Name = {:?}", field.ty, field.ident);
        let fieldname: String = format!("{}", field.ident.clone().unwrap().to_string());
        match field.ty {
            syn::Ty::Path(ref _qselfopt, ref path) => {
                fieldlines.push(typescriptify_named_path(path, &fieldname));
            }
            syn::Ty::Array(ref _b, ref _c) => {
                unimplemented!()
            }
            syn::Ty::Ptr(ref _p) => {
                unimplemented!()
            }
            _ => unimplemented!(),
        }
    }


    let mut s = "".to_string();
    for fieldline in fieldlines {
        s = s + "    " + &fieldline + "\n";
    }
    format!("export interface {} {{\n{}}}", structname, s)
}

fn typescriptify_enum(variant_vec: &Vec<syn::Variant>, enum_name: &str) -> String {
    let mut variants: Vec<String> = Vec::new();
    let mut enum_string: String = "".to_string();
    for variant in variant_vec {
        let mut fieldlines: Vec<String> = vec![];
        let variant_name = format!("{}", variant.ident);
        variants.push(variant_name.to_string());
        let data: &syn::VariantData = &variant.data;

        //add each variant as a field with the variant name as field name, and also its type

        for field in data.fields() {
            //field.ty;    //type of the field
            //field.ident; //name (tuple struct fields have none here)
            //field.vis;   //visibility
            //field.attrs; //attributes
            //println!("Fieldtype = {:?} and Name = {:?}", field.ty, field.ident);
            let fieldname: String = format!("{}", field.ident.clone().unwrap().to_string());
            match field.ty {
                syn::Ty::Array(ref _b, ref _c) => {
                    unimplemented!()
                }
                syn::Ty::Ptr(ref _p) => {
                    unimplemented!()
                }
                syn::Ty::Path(ref _qselfopt, ref path) => {
                    fieldlines.push(typescriptify_named_path(path, &fieldname));
                }
                _ => unimplemented!(),
            }
        }


        let mut s = "".to_string();
        for fieldline in &fieldlines {
            s = s + "    " + fieldline.as_ref() + "\n";
        }

        enum_string = enum_string + &format!("export interface {} {{\n{}}}\n\n", variant_name, s);
    }

    //Add final enum interface:
    let mut s = "".to_string();
    for v in variants {
        s = s + "    " + v.as_ref() + ": " + v.as_ref() + ";\n";
    }
    enum_string + &format!("export interface {} {{\n{}}}\n\n", enum_name, s)
}

fn typescriptify_unnamed_path(path: &syn::Path) -> String {
    let intype: String = format!("{}", path.segments.last().unwrap().ident);
    let generic_params_unformated: &syn::PathParameters = &path.segments.last().clone().unwrap().parameters;
    let mut generics_parameters: Vec<String> = Vec::new();
    match generic_params_unformated {
        &syn::PathParameters::AngleBracketed(ref angle_bracketed_parameter_data) => {

            match intype.as_ref() {
                // TS implements optionals as optional properties on objects.
                // This path isn't named, so it's not an object.
                "Option" =>
                    unreachable!(),
                "Vec" =>
                    format!(
                        "{}[]",
                        typescriptify_parameter_data(angle_bracketed_parameter_data).get(0).unwrap()
                    ),
                _ => { // TODO
                    for ty in &angle_bracketed_parameter_data.types {
                        match ty {
                            &syn::Ty::Path(ref _qotherself, ref qotherpath) => {
                                generics_parameters.push(format!("{}", qotherpath.segments.last().unwrap().ident));
                            }
                            _ => unimplemented!(),
                        }
                    }

                    let mut generic_term_in_angle_brackets: String =
                        if generics_parameters.is_empty() {
                            "".to_string()
                        } else {
                            "<".to_string()
                        };
                    for gen in &generics_parameters {
                        if generic_term_in_angle_brackets.len() > 1 {
                            generic_term_in_angle_brackets = generic_term_in_angle_brackets + ", ";
                        }
                        generic_term_in_angle_brackets =
                            generic_term_in_angle_brackets + &typescriptify_rust_type(gen);
                    }
                    if !generics_parameters.is_empty() {
                        generic_term_in_angle_brackets = generic_term_in_angle_brackets + ">";
                    }
                    typescriptify_rust_type(intype) + &generic_term_in_angle_brackets
                }
            }
        }
        _ => unimplemented!(),
    }
}

fn typescriptify_named_path(path: &syn::Path, field_name: &str) -> String {
    let intype: String = format!("{}", path.segments.last().unwrap().ident);
    let generic_params_unformated: &syn::PathParameters = &path.segments.last().clone().unwrap().parameters;
    match generic_params_unformated {
        &syn::PathParameters::AngleBracketed(ref angle_bracketed_parameter_data) => {

            match intype.as_ref() {
                "Option" => match angle_bracketed_parameter_data.types.first().unwrap() {
                    &syn::Ty::Path(ref _qotherself, ref qotherpath) => format!(
                        "{}?: {};",
                        field_name,
                        typescriptify_unnamed_path(qotherpath)
                    ),
                    _ => unreachable!(),
                },
                _ => format!(
                    "{}: {}",
                    field_name,
                    typescriptify_unnamed_path(path)
                )
            }
        }
        _ => unimplemented!(),
    }
}

fn typescriptify_parameter_data(parameter_data: &syn::AngleBracketedParameterData) -> Vec<String> {
    let mut typescriptified_parameters: Vec<String> = Vec::new();

    for vec_type in &parameter_data.types {
        match vec_type {
            &syn::Ty::Path(ref _qotherself, ref qotherpath) => {
                let segment = typescriptify_path_segment(qotherpath.segments.last().unwrap());
                typescriptified_parameters.push(segment);
            }
            _ => unimplemented!(),
        }
    }

    typescriptified_parameters
}

fn typescriptify_path_segment(path_segment: &syn::PathSegment) -> String {
    let typescript_type = typescriptify_rust_type(&path_segment.ident);
    match typescript_type.as_ref() {
        "number" => "number",
        "boolean" => "boolean",
        "string" => "string",
        "any" => "any",
        a @ _ => a, // TODO fix
    }.to_string()
}
