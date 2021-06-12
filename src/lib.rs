use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Ident, Result, Token};

mod kw {
    syn::custom_keyword!(to);
    syn::custom_keyword!(with);
    syn::custom_keyword!(into);
}

#[proc_macro]
pub fn attach(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AttachInput);
    eprintln!("{:?}", input);

    let tag = &input.tag;
    let parent_field = input.tag.to_string().to_case(Case::Snake);
    let parent_field_ident = Ident::new(&parent_field, Span::call_site());
    let parent = &input.parents[0];

    let tokens = quote! {
        match container[current] {
            Tag::#parent (ref mut parent) => {
                let #parent_field_ident = #tag::new();
                new_tag = Some(Tag::#tag(#parent_field_ident));
                current = container_len;
                parent.#parent_field_ident = Some(current.clone());
                stack.push(current.clone());
            }
            _ => {}
        }
    };
    tokens.into()
}

#[derive(Debug)]
struct AttachInput {
    tag: Ident,
    parents: Vec<Ident>,
    attrs: Vec<Ident>,
}

impl Parse for AttachInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let tag = syn::Ident::parse(input)?;
        let lookahead = input.lookahead1();
        let mut attrs = Vec::new();
        if lookahead.peek(kw::with) {
            let _with = input.parse::<kw::with>()?;
            let punctuated_attrs = Punctuated::<Ident, Token![,]>::parse_separated_nonempty(input)?;
            attrs = punctuated_attrs.into_iter().collect();
        }
        let _to = input.parse::<kw::to>()?;
        let parent = syn::Ident::parse(input)?;
        Ok(AttachInput {
            tag,
            parents: vec![parent],
            attrs,
        })
    }
}

#[proc_macro]
pub fn push(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as PushInput);
    eprintln!("{:?}", input);

    let tag = &input.tag;
    let mut parent_field = input.tag.to_string().to_case(Case::Snake);
    // make plural
    if !parent_field.ends_with("s") {
        parent_field.push_str("s");
    }
    let parent_field_ident = Ident::new(&parent_field, Span::call_site());
    let parent = &input.parents[0];
    let attr_ident = input.attrs;
    let mut attr_str: Vec<String> = Vec::new();
    for ident in &attr_ident {
        attr_str.push(String::from(ident.to_string()));
    }

    let tokens = quote! {
        match container[current] {
            Tag::#parent (ref mut parent) => {
                let mut #parent_field_ident = #tag::new();
                for attribute in attributes {
                    match attribute.name.local_name.as_str() {
                        #(#attr_str => {
                            #parent_field_ident.#attr_ident = Some(attribute.value);
                        })*
                        _ => {}
                    }
                }
                new_tag = Some(Tag::#tag(#parent_field_ident));
                current = container_len;
                parent.#parent_field_ident.push(current.clone());
                stack.push(current.clone());
            }
            _ => {}
        }
    };
    tokens.into()
}

#[derive(Debug)]
struct PushInput {
    tag: Ident,
    parents: Vec<Ident>,
    attrs: Vec<Ident>,
}

impl Parse for PushInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let tag = syn::Ident::parse(input)?;
        let lookahead = input.lookahead1();
        let mut attrs = Vec::new();
        if lookahead.peek(kw::with) {
            let _with = input.parse::<kw::with>()?;
            let punctuated_attrs = Punctuated::<Ident, Token![,]>::parse_separated_nonempty(input)?;
            attrs = punctuated_attrs.into_iter().collect();
        }
        let _to = input.parse::<kw::into>()?;
        let parent = syn::Ident::parse(input)?;

        Ok(PushInput {
            tag,
            parents: vec![parent],
            attrs,
        })
    }
}
#[proc_macro]
pub fn close(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CloseInput);
    eprintln!("{:?}", input);

    let tag = &input.tag;

    let tokens = quote! {
        match container[current] {
            Tag::#tag (ref mut tag_field) => {
                stack.pop();
                current = stack.last().unwrap().to_owned();
                tag_field.parent = Some(current.clone());
            }
            _ => {}
        }
    };
    tokens.into()
}

#[derive(Debug)]
struct CloseInput {
    tag: Ident,
}

impl Parse for CloseInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let tag = syn::Ident::parse(input)?;
        eprintln!("{:?}", tag);
        Ok(CloseInput { tag })
    }
}
