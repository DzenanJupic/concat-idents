extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{
    Block, Ident, LitBool, LitByte, LitByteStr,
    LitChar, LitFloat, LitInt, LitStr, parse_macro_input, Token, visit_mut,
};
use syn::parse::{self, Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::Underscore;
use syn::visit_mut::VisitMut;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let t = trybuild::TestCases::new();
        t.pass("tests/pass.rs");
        t.compile_fail("tests/fail/*.rs");
    }
}

/// A helper struct that implements [`Parse`] and extracts the `replace_ident`, the `concatenated_ident` and
/// the code `block` from the original macro input
/// ```text
/// concat_idents!(
///     ident = ident1, _, ident2 { /* code */ }
///     ^^^^^   ^^^^^^^^^^^^^^^^^ ^^^^^^^^^^^^^^
///     |       |                 code block
///     |       |
///     |       concatenated-ident (in this case: 'ident1_ident2')
///     |
///     replace-ident
/// );
/// ```
struct InputParser {
    replace_ident: Ident,
    concatenated_ident: Ident,
    block: Block,
}

impl Parse for InputParser {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let replace_ident: Ident = input.parse()?;
        let _: Token![=] = input.parse()?;
        let IdentParser(concatenated_ident) = input.parse()?;
        let block: Block = input.parse()?;

        Ok(InputParser {
            replace_ident,
            concatenated_ident,
            block,
        })
    }
}

/// A helper struct that implements [`Parse`] and makes one [`Ident`] from a comma separated list
/// of idents, literals and underscores
/// ```text
/// ident1, ident2, _, 3, _, true
/// => ident1ident2_3_true
/// ```
struct IdentParser(Ident);

impl Parse for IdentParser {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut ident_parts = vec![];

        while !input.peek(syn::token::Brace) {
            ident_parts.push(IdentPart::parse(input)?);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        let span = match ident_parts.first() {
            Some(IdentPart::Ident(i)) => i.span(),
            Some(IdentPart::Underscore(u)) => u.span(),
            Some(IdentPart::Str(s)) => s.span(),
            Some(IdentPart::Char(c)) => c.span(),
            Some(IdentPart::Bool(b)) if ident_parts.len() > 1 => b.span(),

            Some(IdentPart::Bool(b)) => return Err(syn::Error::new(
                b.span(),
                "Identifies cannot consist of only one bool",
            )),
            Some(IdentPart::Int(i)) if ident_parts.len() > 1 => return Err(syn::Error::new(
                i.span(),
                "Identifies cannot start with integers",
            )),
            Some(IdentPart::Int(i)) => return Err(syn::Error::new(
                i.span(),
                "Identifies cannot start nor consist only of integers with integers",
            )),
            None => return Err(syn::Error::new(
                input.span(),
                "Expected at least one identifier",
            ))
        };

        let mut ident = String::new();

        for part in ident_parts {
            match part {
                IdentPart::Ident(i) => ident.push_str(i.to_string().trim_start_matches("r#")),
                IdentPart::Underscore(_) => ident.push('_'),
                IdentPart::Int(i) => ident.push_str(i.to_string().as_str()),
                IdentPart::Bool(b) => ident.push_str(b.value.to_string().as_str()),
                IdentPart::Str(s) => ident.push_str(s.value().as_str()),
                IdentPart::Char(c) => ident.push(c.value())
            }
        }

        Ok(Self(Ident::new(ident.as_str(), span)))
    }
}

/// A helper struct, that represents a valid part of an identifier. Does not guarantee, that
/// this specific part is a fully qualified identifier.
/// 
/// ```text
/// ident1, ident2, _, 3, _, true
/// => ident1, ident2, _, 3, _, true
/// ```
enum IdentPart {
    Underscore(Underscore),
    Ident(Ident),
    Int(LitInt),
    Bool(LitBool),
    Str(LitStr),
    Char(LitChar),
}

impl Parse for IdentPart {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        if input.peek(Ident) {
            Ok(Self::Ident(input.parse()?))
        } else if input.peek(Token![_]) {
            Ok(Self::Underscore(input.parse()?))
        } else if input.peek(LitInt) {
            Ok(Self::Int(input.parse()?))
        } else if input.peek(LitBool) {
            Ok(Self::Bool(input.parse()?))
        } else if input.peek(LitStr) {
            let string = input.parse::<LitStr>()?;
            if string.value().contains(|c: char| !c.is_ascii_alphanumeric() && c != '_') {
                Err(syn::Error::new(
                    string.span(),
                    "string literals can only contain [a-zA-Z0-9_]",
                ))
            } else {
                Ok(Self::Str(string))
            }
        } else if input.peek(LitChar) {
            let char = input.parse::<LitChar>()?;
            let c = char.value();
            if !c.is_ascii_alphanumeric() && c != '_' {
                Err(syn::Error::new(
                    char.span(),
                    "character literals can only contain [a-zA-Z0-9_]",
                ))
            } else {
                Ok(Self::Char(char))
            }
        } else if input.peek(LitByteStr) {
            Err(syn::Error::new(input.span(), "Identifiers cannot contain byte string"))
        } else if input.peek(LitByte) {
            Err(syn::Error::new(input.span(), "Identifiers cannot contain bytes"))
        } else if input.peek(LitFloat) {
            Err(syn::Error::new(input.span(), "Identifiers cannot contain floats"))
        } else {
            Err(syn::Error::new(
                input.span(),
                "Expected either an identifies, a `_`, an int, a bool, \
                 a string-literal, or a character-literal.\n\
                 Note: To create an Identifies from a reserved keywords like `struct`, or `return`, \
                 wrap it quotes, i.e. `\"struct\"`, or escape them with `r#`, i.e. `r#struct` .",
            ))
        }
    }
}

/// A helper struct that implements [`VisitMut`] and is responsible for replacing the `replace_ident`
/// with the `concatenated_ident`.
struct IdentReplacer {
    replace_ident: Ident,
    concatenated_ident: Ident,
    code_block: Option<Block>,
}

impl IdentReplacer {
    /// Creates a new Instance of IdentReplacer from an InputParser
    fn from_input_parser(input_parser: InputParser) -> Self {
        Self {
            replace_ident: input_parser.replace_ident,
            concatenated_ident: input_parser.concatenated_ident,
            code_block: Some(input_parser.block),
        }
    }

    /// Replaces all `replace_idents` in the `code_block` with the `concatenated_ident`
    fn replace_idents(mut self) -> Self {
        let mut code = self.code_block
            .take()
            .unwrap();
        self.visit_block_mut(&mut code);
        self.code_block = Some(code);

        self
    }

    /// generates a TokenStream from the code-block
    fn produce_token_stream(self) -> TokenStream {
        let statements = self.code_block.unwrap().stmts;
        (quote! { #( #statements )* }).into()
    }
}

impl VisitMut for IdentReplacer {
    fn visit_ident_mut(&mut self, node: &mut Ident) {
        if *node == self.replace_ident {
            *node = self.concatenated_ident.clone();
        }

        // Delegate to the default impl
        visit_mut::visit_ident_mut(self, node);
    }
}

/// This macros makes it possible to concatenate identifiers at compile time and use them as normal.
/// It's an extension/replacement of `std::concat_idents`, since in comprassion to the std-solution,
/// the idents here can be used everywhere.
///
/// # Usage:
/// ### Basic usage
/// ```
/// use concat_idents::concat_idents;
///
/// concat_idents!(fn_name = foo, _, bar {
///        fn fn_name() {
///            // --snip--
///        }
/// });
///
/// foo_bar();
/// ```
///
/// ### Generating Tests
/// ```
///# use concat_idents::concat_idents;
///# use std::ops::{Add, Sub};
/// macro_rules! generate_test {
///    ($method:ident($lhs:ident, $rhs:ident)) => {
///        concat_idents!(test_name = $method, _, $lhs, _, $rhs {
///            #[test]
///            fn test_name() {
///                let _ = $lhs::default().$method($rhs::default());
///            }
///        });
///    };
/// }
///
/// #[derive(Default)]
/// struct S(i32);
/// impl Add<i32> for S {
///    // --snip--
///#    type Output = S;
///#    fn add(self,rhs: i32) -> Self::Output { S(self.0 + rhs) }
/// }
/// impl Sub<i32> for S {
///    // --snip--
///#    type Output = S;
///#    fn sub(self,rhs: i32) -> Self::Output { S(self.0 - rhs) }
/// }
///
/// generate_test!(add(S, i32));
/// generate_test!(sub(S, i32));
/// ```
#[proc_macro]
pub fn concat_idents(item: TokenStream) -> TokenStream {
    let input_parser = parse_macro_input!(item as InputParser);

    IdentReplacer::from_input_parser(input_parser)
        .replace_idents()
        .produce_token_stream()
}
