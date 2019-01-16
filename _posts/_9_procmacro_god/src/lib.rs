extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::{Error, Expr, ExprClosure, LitStr, Token, parse};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;

struct Parameters {
    format: LitStr,
    closure: ExprClosure,
}

impl Parse for Parameters {
    fn parse(input: ParseStream) -> Result<Self> {
        let format = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let closure: Expr = input.parse()?;
        match closure {
            Expr::Closure(closure) =>
                Ok(Parameters {
                    format,
                    closure,
                }),
            _ => Err(Error::new(closure.span(), "Argument must be a closure"))
        }
    }
}

#[derive(PartialEq)]
enum Param {
    Unknown,
    Int,
    Long,
    LongInt,
    Repeat(Box<Param>),
}

fn to_code(span: Span, param: &Param, recursive: bool) -> proc_macro2::TokenStream {
    let start =
        if recursive {
            quote! {
            }
        }
        else {
            quote! {
                let word = line.split_whitespace().next().unwrap();
            }
        };
    match param {
        Param::Unknown => quote_spanned! {
            span => compile_error!("invalid format string")
        },
        Param::Int => quote! {{
            #start
            let num = word.parse::<i32>().unwrap();
            closure(num, acc)
        }},
        Param::Long | Param::LongInt => quote! {{
            #start
            let num = word.parse::<i64>().unwrap();
            closure(num, acc)
        }},
        Param::Repeat(ref inner_param) => {
            let inner_code = to_code(span, inner_param, true);
            quote! {{
                for word in line.split_whitespace() {
                    acc = #inner_code;
                }
                acc
            }}
        },
    }
}

#[proc_macro]
pub fn scanfold(input: TokenStream) -> TokenStream {
    let parameters: Result<Parameters> = parse(input);
    let result =
        match parameters {
            Ok(params) => {
                let format = params.format.value();
                let mut is_param = false;
                let mut param = Param::Unknown;
                for char in format.chars() {
                    if is_param {
                        match char {
                            'd' =>
                                if param == Param::Long {
                                    param = Param::LongInt;
                                }
                                else {
                                    param = Param::Int;
                                },
                            'l' => param = Param::Long,
                            '+' => param = Param::Repeat(Box::new(param)),
                            _ => (),
                        }
                    }
                    else if char == '%' {
                        is_param = true;
                    }
                }
                let code = to_code(params.format.span(), &param, false);
                let closure = params.closure;
                quote! {{
                    let closure = #closure;
                    let mut line = String::new();
                    let mut acc = 0;
                    std::io::stdin().read_line(&mut line).unwrap();
                    #code
                }}
            },
            Err(error) => {
                let msg = error.to_string();
                quote_spanned! { error.span() =>
                    compile_error!(#msg)
                }
            },
        };
    result.into()
}
