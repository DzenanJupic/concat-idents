use concat_idents::concat_idents;

concat_idents!(struct_name = Foo, Bar {
    struct struct_name;
});

concat_idents!(enum_name = Bar, Foo {
    enum enum_name {
        A,
        B
    }
});

concat_idents!(fn_name = foo, bar, {
    fn fn_name() {
        println!("works with trailing comma!");
    }
});

concat_idents!(fn_name = _, foo, _, bar {
    fn fn_name() {
        println!("works with underscore!");
    }
});

concat_idents!(fn_name = foo, 1, bar, 2 {
    fn fn_name() {
        println!("works with integers!");
    }
});

concat_idents!(fn_name = false, true {
    fn fn_name() {
        println!("works with bools!");
    }
});

macro_rules! create_test {
    ($ident1:ident, $ident2:ident) => {
        concat_idents!(fn_name = $ident1, _, $ident2 {
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

create_test!(test, function);

fn main() {
    let _ = FooBar;
    let _ = BarFoo::A;
    foobar();
    _foo_bar();
    foo1bar2();
    falsetrue();
}
