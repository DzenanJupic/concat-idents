use concat_idents::concat_idents;

concat_idents!(fn_name = 1, foo, bar, {
    fn fn_name() {
        /* integers are not allowed at the beginning of an ident */
    }
});

concat_idents!(fn_name = true {
    fn fn_name() {
        /* booleans are reserved key-words */
    }
});

concat_idents!(fn_name = struct {
    fn fn_name() {
        /* reserved key-words in general cannot be used as idents */
    }
});

concat_idents!(fn_name = 1 {
    fn fn_name() {
        /* an ident cannot be made of just an integer */
    }
});

concat_idents!(fn_name = foo, 1.0 {
    fn fn_name() {
        /* floats are not allowed in idents */
    }
});

concat_idents!(fn_name = foo, "bar" {
    fn fn_name() {
        /*
        string slices are not allowed to not mislead users to use this macro dynamically
        (at runtime) (would not be possible)
        */
    }
});

concat_idents!(fn_name =  {
    fn fn_name() {
        /* idents cannot be nothing */
    }
});

fn main() {}
