use concat_idents::concat_idents;

concat_idents!(struct_name = Struct, Name {
    struct struct_name;
});

concat_idents!(enum_name = Enum, Name {
    enum enum_name {
        A,
        B
    }
});

concat_idents!(fn_name = works, _, with, _, trailing_, comma, {
    fn fn_name() {
        println!("works with trailing comma!");
    }
});

concat_idents!(fn_name = _, works_, with, _underscores, _ {
    fn fn_name() {
        println!("works with underscores!");
    }
});

concat_idents!(fn_name = __ {
    fn fn_name() {
        println!("works with only underscores!");
    }
});

concat_idents!(fn_name = works, 1, with, 2, ints {
    fn fn_name() {
        println!("works with integers!");
    }
});

concat_idents!(fn_name = false, true {
    fn fn_name() {
        println!("works with bools!");
    }
});

concat_idents!(fn_name = "works_with_", string, _, literals {
    fn fn_name() {
        println!("works with string-literals!");
    }
});

concat_idents!(fn_name = 'w', orks_with, '_', chars {
    fn fn_name() {
        println!("works with character-literals!");
    }
});

concat_idents!(fn_name = r#struct, _, is, _, possible {
    fn fn_name() {
        println!("works with escaped reserved keywords!");
    }
});

concat_idents!(fn_name = "super", _, is, _also_possible {
    fn fn_name() {
        println!("works with quoted reserved keywords!");
    }
});

concat_idents!(fn_name = _works_, r#while, '_', 1, stuff_, is, _, "mixed",   {
    fn fn_name() {
        println!("works with quoted reserved keywords!");
    }
});

macro_rules! nested {
    () => {
        concat_idents!(fn_name = outer {
            concat_idents!(something = inner {
                
            });
        });
    };
}

nested!();


macro_rules! create_test {
    ($ident1:ident, $ident2:ident) => {
        concat_idents!(fn_name = $ident1, _, $ident2 {
            fn fn_name() {
                println!("works in macros!");
            }
        });
        concat_idents!(fn_name = test_, $ident1, _, $ident2 {
            #[test]
            fn fn_name() {
                println!(
                    "use cargo-expand to see me\n\
                    cargo expand --test pass --color=always --theme=GitHub --tests"
                );
            }
        });
    };
}

create_test!(works_in, r#macro);

fn main() {
    let _ = StructName;
    let _ = EnumName::A;
    works_with_trailing_comma();
    _works_with_underscores_();
    __();
    works1with2ints();
    falsetrue();
    works_with_string_literals();
    works_with_chars();
    struct_is_possible();
    super_is_also_possible();
    works_in_macro();
    _works_while_1stuff_is_mixed();
}
