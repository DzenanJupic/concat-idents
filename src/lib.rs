extern crate proc_macro;

use proc_macro::TokenStream;

use quote::{format_ident, ToTokens};
use quote::quote;
use syn::{Block, Ident, Lit, parse_macro_input, Token, visit_mut};
use syn::parse::{self, Parse, ParseStream};
use syn::visit_mut::VisitMut;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let t = trybuild::TestCases::new();
        t.pass("tests/pass.rs");
        t.compile_fail("tests/fail.rs");
    }
}

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

struct IdentParser(Ident);

impl Parse for IdentParser {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut ident: Ident = if input.peek(Ident) {
            input.parse()?
        } else if input.peek(Token![_]) {
            Ident::from(input.parse::<Token![_]>()?)
        } else if input.peek(Lit) {
            let lit: Lit = input.parse()?;
            if !input.peek(Token![,]) {
                return Err(syn::Error::new(lit.span(), "Expected ident"));
            }
            if let Lit::Bool(bool_) = lit {
                let bool_ = bool_.to_token_stream().to_string();
                let _: Token![,] = input.parse()?;
                if input.peek(Lit) {
                    let lit: Lit = input.parse()?;
                    match lit {
                        Lit::Int(i) => format_ident!("{}{}", bool_, i.to_string()),
                        Lit::Bool(b) => format_ident!("{}{}", bool_, b.into_token_stream().to_string()),
                        _ => return Err(syn::Error::new(lit.span(), "Expected ident"))
                    }
                } else {
                    let Self(ident) = Self::parse(input)?;
                    return Ok(Self(format_ident!("{}{}", bool_, ident)));
                }
            } else {
                return Err(syn::Error::new(lit.span(), "Expected ident"));
            }
        } else {
            return Err(syn::Error::new(input.span(), "Expected ident"));
        };

        while parse_ident(input, &mut ident)? {}

        Ok(Self(ident))
    }
}

fn parse_ident(parse_stream: ParseStream, ident: &mut Ident) -> parse::Result<bool> {
    if parse_stream.peek(Token![,]) {
        let _: Token![,] = parse_stream.parse()?;
    }

    if parse_stream.peek(Ident) {
        let next_ident: Ident = parse_stream.parse()?;
        *ident = format_ident!("{}{}", ident, next_ident);
    } else if parse_stream.peek(Token![_]) {
        let _: Token![_] = parse_stream.parse()?;
        *ident = format_ident!("{}_", ident);
    } else if parse_stream.peek(Lit) {
        let lit: Lit = parse_stream.parse()?;
        match lit {
            Lit::Int(i) => *ident = format_ident!("{}{}", ident, i.to_string()),
            Lit::Bool(b) => *ident = format_ident!("{}{}", ident, b.into_token_stream().to_string()),
            _ => return Err(syn::Error::new(lit.span(), "Expected ident"))
        }
    } else {
        if parse_stream.peek(Token![,]) {
            let _: Token![,] = parse_stream.parse()?;
        }
        return Ok(false);
    }

    Ok(true)
}

struct IdentReplacer {
    replace_ident: Ident,
    concatenated_ident: Ident,
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

/// # Basic usage:
///     concat_idents!(
///         <IDENT> = ident1, ident2, ident3, ... {
///             fn <IDENT>() {
///                 /* some code */
///             }
///         }
///     );
#[proc_macro]
pub fn concat_idents(item: TokenStream) -> TokenStream {
    let mut parsed_input = parse_macro_input!(item as InputParser);

    let mut ident_replacer = IdentReplacer {
        replace_ident: parsed_input.replace_ident,
        concatenated_ident: parsed_input.concatenated_ident,
    };
    ident_replacer.visit_block_mut(&mut parsed_input.block);

    let statements = parsed_input.block.stmts;
    (quote! { #( #statements )* }).into()
}
