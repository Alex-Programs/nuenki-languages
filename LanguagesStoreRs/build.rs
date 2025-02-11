use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::fs;
use std::path::Path;
use toml::Value;

fn main() {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("language_config.rs");

    let toml_str = fs::read_to_string("../languages.toml").expect("Failed to read languages.toml");
    let value: Value = toml::from_str(&toml_str).expect("Failed to parse TOML");

    let mut variants = Vec::new();
    let mut from_wiktionary = Vec::new();
    let mut to_wiktionary = Vec::new();
    let mut to_wiktionary_long = Vec::new();
    let mut to_technical = Vec::new();
    let mut to_llm = Vec::new();
    let mut to_deepl = Vec::new();
    let mut supports_deepl = Vec::new();
    let mut to_database = Vec::new();
    let mut to_native = Vec::new();
    let mut groq_acceptable = Vec::new();

    for (key, lang) in value.as_table().unwrap() {
        let lang = lang.as_table().unwrap();
        let technical_name = key;
        let variant = Ident::new(technical_name, proc_macro2::Span::call_site());

        variants.push(quote! { #variant });

        let wiktionary_code = lang["wiktionary_code"].as_str().unwrap();
        from_wiktionary.push(quote! { #wiktionary_code => Some(TargetLanguage::#variant) });
        to_wiktionary.push(quote! { TargetLanguage::#variant => #wiktionary_code.to_string() });

        let wiktionary_name = lang["wiktionary_name"].as_str().unwrap();
        to_wiktionary_long.push(quote! { TargetLanguage::#variant => #wiktionary_name });

        to_technical.push(quote! { TargetLanguage::#variant => #technical_name });

        let llm_name = lang["llm_name"].as_str().unwrap();
        to_llm.push(quote! { TargetLanguage::#variant => #llm_name });

        let native_name = lang["native_name"].as_str().unwrap();
        to_native.push(quote! { TargetLanguage::#variant => #native_name });

        let is_groq_acceptable_val = lang["is_groq_acceptable"].as_bool().unwrap();
        groq_acceptable.push(quote! { TargetLanguage::#variant => #is_groq_acceptable_val });

        let deepl_name = lang
            .get("deepl_name")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        to_deepl.push(quote! { TargetLanguage::#variant => #deepl_name });

        let supports_deepl_val = lang["supports_deepl"].as_bool().unwrap();
        supports_deepl.push(quote! { TargetLanguage::#variant => #supports_deepl_val });

        let database_name = lang["database_name"].as_str().unwrap();
        to_database.push(quote! { TargetLanguage::#variant => #database_name });
    }

    let generated = quote! {
        #[derive(Serialize, Deserialize, Debug, EnumIter, PartialEq, Eq, Hash, Clone, Savefile)]
        pub enum TargetLanguage {
            #(#variants),*
        }

        impl TargetLanguage {
            pub fn from_wiktionary_language_code_n(code: &str) -> Option<Self> {
                match code {
                    #(#from_wiktionary,)*
                    _ => None,
                }
            }

            pub fn to_wiktionary_language_code_n(&self) -> String {
                match self {
                    #(#to_wiktionary,)*
                }
            }

            pub fn to_wiktionary_long_name_n(&self) -> &'static str {
                match self {
                    #(#to_wiktionary_long,)*
                }
            }

            pub fn to_extension_technical_format_n(&self) -> &'static str {
                match self {
                    #(#to_technical,)*
                }
            }

            pub fn to_llm_format_n(&self) -> &'static str {
                match self {
                    #(#to_llm,)*
                }
            }

            pub fn to_deepl_format_n(&self) -> &'static str {
                match self {
                    #(#to_deepl,)*
                }
            }

            pub fn to_native_name_n(&self) -> &'static str {
                match self {
                    #(#to_native,)*
                }
            }

            pub fn supports_deepl_n(&self) -> bool {
                match self {
                    #(#supports_deepl,)*
                }
            }

            pub fn is_groq_acceptable(&self) -> bool {
                match self {
                    #(#groq_acceptable,)*
                }
            }

            pub fn to_database_format_n(&self) -> &'static str {
                match self {
                    #(#to_database,)*
                }
            }
        }
    };

    fs::write(dest_path, generated.to_string()).expect("Failed to write generated code");
}
