# concat-idents!
This crate provides a single, easy to use macro

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
concat_idents!(ident = foo, bar);
...
``` 
2. one of the identifiers is invalid
```rust
concat_idents!(ident = true {});
concat_idents!(ident = 1 {});
concat_idents!(ident = foo, 1.0 {});
concat_idents!(ident =  {});
...
``` 
