use quote::quote;
use syn::ItemStruct;

pub fn getters(item: &ItemStruct) -> proc_macro2::TokenStream {

    let struct_name = &item.ident;
    let generics = &item.generics;
    let where_clause = &item.generics.where_clause;

    let mut getters = quote! {};
    for field in &item.fields {
        if let Some(field_name) = &field.ident {
            let field_type = &field.ty;
            getters = quote! {
                #getters
                pub fn #field_name(&mut self) -> &mut #field_type
                {
                    &mut self.#field_name
                }
            }
        }
    }

    quote! {
        impl #generics #struct_name #where_clause {
            #getters
        }
    }
}