use quote::quote;
use syn::ItemStruct;

pub(crate) fn gnu_command_factory(item: &ItemStruct) -> proc_macro2::TokenStream {

    let struct_name = &item.ident;
    let generics = &item.generics;
    let where_clause = &item.generics.where_clause;

    let mut implementation = quote!{};
    for field in &item.fields {
        let field_name = &field.ident;
        implementation = quote! {
            #implementation

            if let Ok(mut new_commands) = GnuCommandFactory::as_commands(&self.#field_name) {
                commands.append(&mut new_commands);
            }
        };
    }
    implementation = quote! {
        impl #generics GnuCommandFactory for #struct_name #where_clause {
            fn as_commands(&self) -> GnuCommandFactoryResult {
                let mut commands = VecDeque::new();
                #implementation

                Ok(commands)
            }
        }
    };

    implementation
}