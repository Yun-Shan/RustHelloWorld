extern crate proc_macro;

use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens};
use syn::{AttributeArgs, FnArg, ItemFn, ItemStruct, MetaNameValue, parse_macro_input};

use proc_macro::TokenStream as CompilerTokenStream;


fn print_meta_name_value(prefix: &str, meta: MetaNameValue) {
    let name = meta.path.get_ident().unwrap();
    let value = match meta.lit {
        syn::Lit::Str(lit) => {
            lit.value()
        }
        syn::Lit::Int(lit) => {
            String::from(lit.base10_digits())
        }
        syn::Lit::Bool(lit) => {
            lit.value.to_string()
        }
        syn::Lit::Float(lit) => {
            String::from(lit.base10_digits())
        }
        syn::Lit::Char(lit) => {
            lit.value().to_string()
        }
        _ => panic!("unsupported value type"),
    };

    println!("{} : {} = {}", prefix, name, value);
}

#[proc_macro_attribute]
pub fn entity(macro_arg_attr: CompilerTokenStream, macro_arg_struct: CompilerTokenStream) -> CompilerTokenStream {
    let token_struct = TokenStream2::from(macro_arg_struct);
    let entity_struct = syn::parse2::<ItemStruct>(token_struct).unwrap();

    let mut output = TokenStream2::new();
    output.extend(entity_struct.to_token_stream());
    output.extend(quote! {

startup::on_startup! {
    entity_center::register(|backend| {
        println!("{}", backend.build(sea_orm::Schema::new(backend).create_table_from_entity(Entity).if_not_exists()).sql);
    });
}

    });
    output.into()
}

#[proc_macro_attribute]
pub fn post(macro_arg_attr: CompilerTokenStream, macro_arg_func: CompilerTokenStream) -> CompilerTokenStream {
    //println!("attr: \"{}\"", attrArg.to_string());
    //println!("func : \"{}\"", funcArg.to_string());

    println!("======开始解析=======");
    let token_func = TokenStream2::from(macro_arg_func);
    let func = syn::parse2::<ItemFn>(token_func).unwrap();

    //println!("itemFn: {:?}", func);
    let vis = func.vis;
    let signature = func.sig;
    let ident = signature.ident;
    let arguments = signature.inputs;
    let return_type = signature.output;

    let fn_name = format!("函数 {} 的属性", ident);
    let fn_attr = parse_macro_input!(macro_arg_attr as AttributeArgs);
    fn_attr.iter().for_each(|meta| match meta {
        syn::NestedMeta::Meta(syn::Meta::NameValue(attr)) => {
            print_meta_name_value(&*fn_name, attr.clone());
        }
        _ => panic!("unsupported attr"),
    });

    let new_arguments = &mut arguments.clone();
    new_arguments.clear();
    let args = arguments.iter().map(|fn_arg| match fn_arg {
        FnArg::Typed(syn::PatType { pat, .. }) => match &**pat {
            syn::Pat::Ident(ident) => {
                //attrs.first().unwrap().path.segments;
                //attrs.clear();
                let mut new_arg = fn_arg.clone();
                match new_arg {
                    FnArg::Typed(syn::PatType { ref mut attrs, .. }) => {
                        attrs.iter().for_each(|attr| {
                            let stream = match attr.clone().tokens.into_iter().next().unwrap() {
                                TokenTree::Group(group) => {
                                    group.stream()
                                }
                                _ => panic!("argument pattern is not a simple ident"),
                            };
                            let stream0: proc_macro::TokenStream = proc_macro::TokenStream::from(stream);
                            let arg_attr = crate::parse_macro_input::parse::<AttributeArgs>(stream0).unwrap();
                            let arg_name = format!("参数 {} 的属性{}", ident.ident, attr.path.get_ident().unwrap());
                            arg_attr.iter().for_each(|meta| match meta {
                                syn::NestedMeta::Meta(syn::Meta::NameValue(attr)) => {
                                    print_meta_name_value(&*arg_name, attr.clone());
                                }
                                _ => panic!("unsupported attr"),
                            });
                        });
                        attrs.clear();
                    }
                    _ => panic!("argument pattern is not a simple ident"),
                }
                new_arguments.push(new_arg);
                ident
            }
            _ => panic!("argument pattern is not a simple ident"),
        }
        FnArg::Receiver(_) => panic!("argument is a receiver"),
    }).collect::<Vec<_>>();

    let block = func.block;
    let output = quote! {
        #vis fn #ident ( #new_arguments ) #return_type {
            println!("enhanced function {:?}", (#(#args),*));
            #block
        }
    };
    let out = CompilerTokenStream::from(output);
    //println!("output: \"{}\"", out.to_string());
    println!("======完成解析=======");
    out
}