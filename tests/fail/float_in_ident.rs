concat_idents::concat_idents!(fn_name = foo, 1.0 {
    fn fn_name() {
        /* floats are not allowed in idents */
    }
});

fn main() {}
