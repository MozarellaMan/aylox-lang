use std::{fs::File, io::Write, path::Path};

use codegen::{Enum, Scope};

extern crate codegen;

pub fn generate_ast(base_name: &str, types: &[&str]) -> String {
    let mut scope = Scope::new();
    scope.import("crate::token", "Token");

    let mut base_enum = Enum::new(base_name);
    base_enum.vis("pub").derive("Debug").derive("Clone");
    let mut structs = Vec::new();
    for _type in types.iter() {
        if _type.contains('/') {
            let enum_name: &str = _type
                .split('/')
                .collect::<Vec<&str>>()
                .get(0)
                .unwrap()
                .trim();
            let variants: &str = _type
                .split('/')
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .trim();
            define_enum_type(&mut scope, base_name, enum_name, variants);
        } else if _type.contains(':') {
            let struct_name: &str = _type
                .split(':')
                .collect::<Vec<&str>>()
                .get(0)
                .unwrap()
                .trim();
            let fields: &str = _type
                .split(':')
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .trim();
            structs.push(struct_name);
            base_enum.new_variant(struct_name).tuple(struct_name);
            define_struct_type(&mut scope, base_name, struct_name, fields);
        }
    }
    scope.push_enum(base_enum);

    let visitor = scope.new_trait("Visitor").generic("T").vis("pub");
    for _struct in structs.iter() {
        let func_name = format!("visit_{}", _struct.to_lowercase());
        visitor
            .new_fn(&func_name)
            .arg_mut_self()
            .arg(&_struct.to_lowercase(), *_struct)
            .ret("T");
    }

    scope.to_string()
}

fn define_enum_type(scope: &mut Scope, base_name: &str, enum_name: &str, variants: &str) {
    let variants: Vec<&str> = variants.split(", ").collect();
    let new_enum_type = scope
        .new_enum(enum_name)
        .vis("pub")
        .derive("Debug")
        .derive("Clone");
    for variant in variants.iter() {
        let variant_name: &str = variant
            .split(' ')
            .collect::<Vec<&str>>()
            .get(0)
            .unwrap()
            .trim();
        let variant_type: &str = variant
            .split(' ')
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .trim();
        let variant_type: String = if variant_type == base_name {
            format!("Box<{}>", base_name)
        } else {
            variant_type.to_string()
        };
        new_enum_type.new_variant(variant_name).tuple(&variant_type);
    }
}

fn define_struct_type(scope: &mut Scope, base_name: &str, struct_name: &str, fields: &str) {
    let fields: Vec<&str> = fields.split(", ").collect();
    let new_struct = scope
        .new_struct(struct_name)
        .vis("pub")
        .derive("Debug")
        .derive("Clone")
        .derive("new");
    for field in fields.iter() {
        let field_type: &str = field
            .split(' ')
            .collect::<Vec<&str>>()
            .get(0)
            .unwrap()
            .trim();
        let field_name: &str = field
            .split(' ')
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .trim();
        let field_type: String = if field_type == base_name {
            format!("Box<{}>", field_type)
        } else {
            field_type.to_owned()
        };
        new_struct.field(field_name, field_type);
    }
}

fn main() {
    let output_path =
        Path::new("C:/Users/ayoez/Documents/Rust-Projects/language-dev/aylox-lang/src/ast.rs");
    let base_name = "Expr";
    let type_list = [
        "LiteralVal / String String, Number f64",
        "Binary     : Expr left, Token operator, Expr right",
        "Grouping   : Expr expression",
        "Literal    : LiteralVal value",
        "Unary      : Token operator, Expr right",
    ];

    let mut file = match File::create(output_path) {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    };

    match file.write_all(generate_ast(base_name, &type_list).as_bytes()) {
        Ok(_) => {
            println!("Succesfully written to {}", output_path.display())
        }
        Err(why) => {
            panic!("couldn't write to {}: {}", output_path.display(), why)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::generate_ast;
    use indoc::indoc;

    #[test]
    fn input_generates_correct_code() {
        let base_name = "Expr";
        let type_list = [
            "LiteralVal / String String, Number f64",
            "Binary     : Expr left, Token operator, Expr right",
            "Grouping   : Expr expression",
            "Literal    : LiteralVal value",
            "Unary      : Token operator, Expr right",
        ];
        let expected = indoc! {"
        use crate::token::Token;
        #[derive(Debug, Clone)]
        pub enum LiteralVal {
            String(String),
            Number(f64),
        }
        #[derive(Debug, Clone, new)]
        pub struct Binary {
            left: Box<Expr>,
            operator: Token,
            right: Box<Expr>,
        }
        #[derive(Debug, Clone, new)]
        pub struct Grouping {
            expression: Box<Expr>,
        }
        #[derive(Debug, Clone, new)]
        pub struct Literal {
            value: LiteralVal,
        }
        #[derive(Debug, Clone, new)]
        pub struct Unary {
            operator: Token,
            right: Box<Expr>,
        }
        #[derive(Debug, Clone)]
        pub enum Expr {
            Binary(Binary),
            Grouping(Grouping),
            Literal(Literal),
            Unary(Unary),
        }
        "};

        let actual = generate_ast(base_name, &type_list);
        //println!("{}", actual);
        assert_eq!(
            expected.split_whitespace().collect::<Vec<&str>>(),
            actual.split_whitespace().collect::<Vec<&str>>()
        );
    }
}
