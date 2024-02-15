use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(BadRequestResponder)]
pub fn bad_request_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let gen = quote! {
        #[cfg(target_family = "unix")]
        impl actix_web::Responder for #name {
            type Body = actix_web::body::BoxBody;
            fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
                let mut response = match self.error {
                    Some(_) => actix_web::HttpResponse::BadRequest(),
                    None => actix_web::HttpResponse::Ok(),
                };

                response
                    .content_type(actix_web::http::header::ContentType::json())
                    .json(self)
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(DefaultWithError)]
pub fn default_with_error_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let gen = quote! {
        impl Default for #name {
            fn default() -> Self {
                #name { error: None }
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(ErrorBadRequestResponder)]
pub fn error_bad_request_responder_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let name_str = name.to_string();
    if !name_str.ends_with("Error") {
        panic!("Must end with Error");
    }
    let response_struct_name = name_str.strip_suffix("Error").unwrap();
    let response_struct_ident = syn::Ident::new(&response_struct_name, name.span());

    let gen = quote! {
        #[cfg(target_family = "unix")]
        impl actix_web::ResponseError for #name {
            fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
                let s = #response_struct_ident {
                    error: Some(self.clone()),
                };
                actix_web::HttpResponse::BadRequest()
                    .content_type(actix_web::http::header::ContentType::json())
                    .json(s)
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(FromDieselError)]
pub fn from_diesel_error_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let gen = quote! {
        #[cfg(target_family = "unix")]
        impl From<diesel::result::Error> for #name {
            fn from(_: diesel::result::Error) -> Self {
                #name::DBError
            }
        }
    };

    gen.into()
}
