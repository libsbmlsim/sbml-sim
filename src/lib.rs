use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, ExprCast, Ident, Result, Token, Type};

mod kw {
    syn::custom_keyword!(to);
    syn::custom_keyword!(with);
    syn::custom_keyword!(into);
}

#[proc_macro]
pub fn attach(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AttachInput);
    //println!("{:?}", input);

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
    //println!("{:?}", input);

    let tag = &input.tag;
    let mut parent_field = input.tag.to_string().to_case(Case::Snake);
    // make plural
    if !parent_field.ends_with("s") {
        parent_field.push_str("s");
    }
    // identifier for this object in it's parent
    let parent_field_ident = Ident::new(&parent_field, Span::call_site());
    let parent = &input.parents[0];

    // attributes field names and types
    let attr_idents = input.attr_idents;
    let attr_types = input.attr_types;

    // also need strings for matching tokens
    let mut attr_str: Vec<String> = Vec::new();
    for ident in &attr_idents {
        attr_str.push(String::from(ident.to_string()));
    }

    let tokens = quote! {
        // match the current tag
        match container[current] {
            // with the parent
            // TODO: repeat for multiple possible parents
            Tag::#parent (ref mut parent) => {
                // instantiate object of the tag that was found
                let mut #parent_field_ident = #tag::new();
                // parse any attributes, keeping their types in mind
                for attribute in attributes {
                    match attribute.name.local_name.as_str() {
                        #(#attr_str => {
                            #parent_field_ident.#attr_idents = Some(attribute.value.parse::<#attr_types>().expect("Incorrect type"));
                        })*
                        _ => {}
                    }
                }
                // create Tag enum object
                new_tag = Some(Tag::#tag(#parent_field_ident));
                // update current pointer (which is really an int)
                current = container_len;
                // update parent pointer of new tag
                parent.#parent_field_ident.push(current.clone());
                // push current pointer to stack
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
    attr_idents: Vec<Ident>,
    attr_types: Vec<Type>,
}

impl Parse for PushInput {
    fn parse(input: ParseStream) -> Result<Self> {
        //println!("{:#?}", input);
        // parse tag
        let tag = syn::Ident::parse(input)?;
        // define lookahead function
        let mut lookahead = input.lookahead1();
        // define fields used later
        let mut attr_idents = Vec::new();
        let mut attr_types = Vec::new();

        // if attributes are specified
        if lookahead.peek(kw::with) {
            let _with = input.parse::<kw::with>()?;

            // loop over attributes and types
            loop {
                // parse attribute field name as ident
                let ident = syn::Ident::parse(input)?;
                attr_idents.push(ident);
                let _as = input.parse::<Token![as]>();
                // parse attribute type
                let ty = syn::Type::parse(input)?;
                attr_types.push(ty);

                // consume comma if it exists
                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                }

                // break if found into
                // lookahead works only once
                lookahead = input.lookahead1();
                if lookahead.peek(kw::into) {
                    break;
                }
            }
        }
        let _into = input.parse::<kw::into>()?;

        // parse parent
        let parent = syn::Ident::parse(input)?;

        Ok(PushInput {
            tag,
            parents: vec![parent],
            attr_idents,
            attr_types,
        })
    }
}
#[proc_macro]
pub fn close(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CloseInput);
    //println!("{:?}", input);

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
        Ok(CloseInput { tag })
    }
}
