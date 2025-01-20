// In your procedural macro crate (simplified)
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyConfig)]
pub fn my_config_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Read and parse your YAML file (error handling omitted for brevity)
    let config_str = std::fs::read_to_string("config.yaml").unwrap();
    let config_data: serde_yml::Value = serde_yml::from_str(&config_str).unwrap();

    let pods = config_data.get("pods").unwrap().as_mapping().unwrap();

    let pod_structs = pods.iter().map(|(pod_name, pod_data)| {
        let pod_name_str = pod_name.as_str().unwrap();
        let pod_ident = syn::Ident::new(pod_name_str, proc_macro2::Span::call_site());

        let measurements = pod_data.get("measurements").unwrap().as_mapping().unwrap();

        let measurement_fields = measurements
            .iter()
            .map(|(measurement_name, measurement_data)| {
                let measurement_name_str = measurement_name.as_str().unwrap();
                let measurement_ident =
                    syn::Ident::new(measurement_name_str, proc_macro2::Span::call_site());

                let name = measurement_data.get("name").unwrap().as_str().unwrap();
                // ... get other fields (unit, format, limits, etc.)

                quote! {
                    #measurement_ident: Measurement {
                        name: #name.to_string(), //Simplified
                        // ... other fields
                    },
                }
            });

        quote! {
            #pod_ident: Pod {
                // ... other pod fields
                measurements: Measurements {
                    #(#measurement_fields)*
                },
            },
        }
    });

    let expanded = quote! {
        struct Measurements {
            #(#measurement_fields)*
        }
        struct Pod {
            measurements: Measurements,
        }
        struct Config {
            #(#pod_structs)*
        }

        impl Config {
            pub const fn new() -> Self {
                Self {
                    #(#pod_structs)*
                }
            }
        }
    };

    expanded.into()
}
