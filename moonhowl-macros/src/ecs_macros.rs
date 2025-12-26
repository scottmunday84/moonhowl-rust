use crate::prelude::*;

#[proc_macro_attribute]
pub fn ecs_component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    quote! {
        #input

        impl crate::IComponent for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    }
    .into()
}
