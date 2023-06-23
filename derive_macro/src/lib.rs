use proc_macro::TokenStream;
use syn::{DeriveInput, Ident, parse_macro_input};
use quote::{quote, format_ident};

#[proc_macro_derive(EnumMap)]
pub fn derive_map(input: TokenStream) -> TokenStream {
	let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

	let syn::Data::Enum(enm) = ast.data else {
		panic!("This macro can only be used on an enum")
	};

	let name = ast.ident;
	// the name of the type will always be `{enum_name}Map`
	let map_name = format_ident!("{}Map", name.to_string());
	
	let variants: Vec<&Ident> = enm.variants.iter().map(|v| &v.ident).collect();

	// converting all the variants to snake_case from pascalcase (just assuming they're following
	// rust naming standards; a bit naive maybe)
	let members: Vec<Ident> = variants.iter()
		.map(|ident| format_ident!("{}",
			ident.to_string()
				.chars()
				.enumerate()
				.fold(
					String::new(),
					| mut s, (i, c) | {
						if c.is_uppercase() && i > 0 {
							s.push('-');
						}
						s.push(c.to_ascii_lowercase());
						s
					}
				)
		))
		.collect();

	// Create these iteratively since we need to reference two variables for each one of the
	// `quote!{}` calls in these maps which makes them hard to use inside a quote macro with the
	// iteration ( #(#)* ) syntax
	let get_matches: Vec<proc_macro2::TokenStream> = variants.iter()
		.zip(members.iter())
		.map(|(v, m)| quote!{
			#name::#v => &self.#m,
		})
		.collect();

	let set_matches: Vec<proc_macro2::TokenStream> = variants.iter()
		.zip(members.iter())
		.map(|(v, m)| quote!{
			#name::#v => self.#m = new_value,
		})
		.collect();
	
	// Construct and return the final bit of code that we want to generate
	quote!{
		struct #map_name<T> {
			#(#members: T,)*
		}

		impl<T> #map_name<T> {
			// A new function if they don't want to use explicit member instruction
			fn new(
				#(#members: T,)*
			) -> #map_name<T> {
				#map_name {
					#(#members,)*
				}
			}

			// E.g. so that if you're using EnumIter as well, these functions work nicely
			fn get(&self, variant: #name) -> &T {
				match variant {
					#(#get_matches)*
				}
			}

			fn set(&mut self, variant: #name, new_value: T) {
				match variant {
					#(#set_matches)*
				}
			}
		}
	}.into()
}
