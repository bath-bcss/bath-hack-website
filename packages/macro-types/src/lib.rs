use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(JsonResponder)]
pub fn json_responder_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let gen = quote! {
        #[cfg(target_family = "unix")]
        impl actix_web::Responder for #name {
            type Body = actix_web::body::BoxBody;
            fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
                actix_web::HttpResponse::Ok().json(self)
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

#[proc_macro_derive(FromBlockingError)]
pub fn from_blocking_error_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let gen = quote! {
        #[cfg(target_family = "unix")]
        impl From<actix_web::error::BlockingError> for #name {
            fn from(_: actix_web::error::BlockingError) -> Self {
                Self::DBError
            }
        }
    };

    gen.into()
}


#[proc_macro_derive(ResponseError)]
pub fn response_error_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let gen = quote! {
        #[cfg(target_family = "unix")]
        impl actix_web::ResponseError for #name {
            fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
                actix_web::HttpResponse::BadRequest().json(self)
            }
        }
    };

    gen.into()
}
