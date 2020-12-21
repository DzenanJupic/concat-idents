concat_idents::concat_idents!(fn_name = 1, foo, bar, {
    fn fn_name() {
        /* integers are not allowed at the beginning of an ident */
    }
});

fn main() {}
