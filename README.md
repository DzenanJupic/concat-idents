# concat-idents!

![crates.io](https://img.shields.io/crates/v/concat-idents)
![docs.rs](https://docs.rs/concat-idents/badge.svg)
![licence](https://img.shields.io/crates/l/concat-idents)

This crate provides a single, easy to use macro.

## Usage

### Basic usage

 ```rust
 use concat_idents::concat_idents;

 concat_idents!(fn_name = foo, _, bar {
        fn fn_name() {
            // --snip--
        }
 });

 foo_bar();
 ```

### Allowed identifier parts

```rust
concat_idents!(fn_name = _, _ { /* underscores */ });
concat_idents!(fn_name = foo, bar { /* identifiers */ });
concat_idents!(fn_name = "foo", "bar" { /* strings */ });
concat_idents!(fn_name = 'f', 'o', 'o' { /* characters */ });
concat_idents!(fn_name = foo, 1, bar, 2 { /* integers */ });
concat_idents!(fn_name = true, false { /* booleans */ });
concat_idents!(fn_name = "enum", bar { /* quoted reserved keywords (recommended way) */ });
concat_idents!(fn_name = r#struct, bar { /* escaped reserved keywords (not recommended, since some keywords produce error) */ });
```

### Generating Tests

```rust
macro_rules! generate_test {
   ($method:ident($lhs:ident, $rhs:ident)) => {
       concat_idents!(test_name = $method, _, $lhs, _, $rhs {
           #[test]
           fn test_name() {
               let _ = $lhs::default().$method($rhs::default());
           }
       });
   };
}

#[derive(Default)]
struct S(i32);

impl Add<i32> for S {
   type Output = S;
   fn add(self,rhs: i32) -> Self::Output { S(self.0 + rhs) }
}

impl Sub<i32> for S {
   type Output = S;
   fn sub(self,rhs: i32) -> Self::Output { S(self.0 - rhs) }
}

generate_test!(add(S, i32));
generate_test!(sub(S, i32));
```

## Error
This macro will throw a compile error, if:
1. an unexpected syntax is passed

```rust
concat_idents!({});
concat_idents!(ident {});
concat_idents!(ident =  {});
concat_idents!(= foo, bar {});
concat_idents!(ident = foo, bar);
...
``` 
2. one of the identifiers is invalid

```rust
concat_idents!(ident = true {});        // identifiers cannot consist of only one bool 
concat_idents!(ident = 1 {});           // identifiers cannot consist of only one int
concat_idents!(ident = 1, foo {});      // identifiers cannot start with an int
concat_idents!(ident = foo, 1.0 {});    // identifiers cannot contain floats
concat_idents!(ident = "enum");         // identifiers cannot consist of only one reserved keyword
concat_idents!(ident = r#struct);       // identifiers cannot consist of only one reserved keyword
concat_idents!(ident = " space");       // identifiers cannot contain spaces
concat_idents!(ident = "foo-bar🧨");    // identifiers can only contain [a-zA-Z0-9_]
...
``` 
