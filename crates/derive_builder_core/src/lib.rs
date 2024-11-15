//! Internal helper library for the `derive_builder` crate.
//!
//! **Important Note**:
//!
//! * You are probably looking for the [`derive_builder`] crate,
//!   which wraps this crate and is much more ergonomic to use.
//!
//! ## Purpose
//!
//! This is an internal helper library of [`derive_builder`], which allows for
//! all the logic of builder creation to be decoupled from the proc-macro entry
//! point.
//!
//!
//! [`derive_builder`]: https://!crates.io/crates/derive_builder
//! [`derive_builder_core`]: https://!crates.io/crates/derive_builder_core

// #![deny(warnings, missing_docs)]
#![cfg_attr(test, recursion_limit = "100")]
#![feature(let_chains)]
#[macro_use]
extern crate darling;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

mod block;
mod build_method;
mod builder;
mod builder_field;
mod change_span;
mod default_expression;
mod doc_comment;
mod initializer;
mod macro_options;
mod options;
mod setter;

pub(crate) use block::BlockContents;
pub(crate) use build_method::BuildMethod;
pub(crate) use builder::Builder;
pub(crate) use builder_field::{BuilderField, BuilderFieldType};
pub(crate) use change_span::change_span;
use darling::FromDeriveInput;
pub(crate) use default_expression::DefaultExpression;
pub(crate) use doc_comment::doc_comment_from;
pub(crate) use initializer::{FieldConversion, Initializer};
pub(crate) use options::{BuilderPattern, Each};
use quote::TokenStreamExt;
pub(crate) use setter::Setter;

const DEFAULT_STRUCT_NAME: &str = "__default";

/// Derive a builder for a struct
pub fn builder_for_struct(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let opts = match macro_options::Options::from_derive_input(&ast) {
        Ok(val) => val,
        Err(err) => {
            return err.write_errors();
        }
    };
    let model = opts.ident.clone();

    let schema = format_ident!("{}", model.to_string().to_snake_case());
    let get = format_ident!("get_{schema}_by_id");
    let create = format_ident!("create_{schema}");
    let delete = format_ident!("delete_{schema}_by_id");
    let update = format_ident!("update_{schema}_by_id");
    let page = format_ident!("page_of_{schema}");
    let schema_s = format_ident!("{}s", schema);
    let new_model = format_ident!("New{}", model);
    let mut builder = opts.as_builder();
    let builder_ident = opts.builder_ident();
    // let  build_fn = opts.as_build_method();

    // builder.doc_comment(format!(
    //     include_str!("doc_tpl/builder_struct.md"),
    //     struct_name = ast.ident
    // ));
    // build_fn.doc_comment(format!(
    //     include_str!("doc_tpl/builder_method.md"),
    //     struct_name = ast.ident
    // ));

    let mut filters = vec![];
    for field in opts.fields() {
        let ident = field.field_ident();
        filters.push(quote!(
                    if let Some(filter_param) = filter.#ident {
                        match filter_param.compare {
                            Compare::NotEqual => {
                                statement = statement.filter(crate::schema::#schema_s::#ident.ne(filter_param.compare_value.clone()));
                                count_statement = count_statement.filter(crate::schema::#schema_s::#ident.ne(filter_param.compare_value));
                            }
                            Compare::Equal => {
                                statement = statement.filter(crate::schema::#schema_s::#ident.eq(filter_param.compare_value.clone()));
                                count_statement = count_statement.filter(crate::schema::#schema_s::#ident.eq(filter_param.compare_value));
                            }
                            Compare::Greater => {
                                statement = statement.filter(crate::schema::#schema_s::#ident.gt(filter_param.compare_value.clone()));
                                count_statement = count_statement.filter(crate::schema::#schema_s::#ident.gt(filter_param.compare_value));
                            }
                            Compare::GreaterAndEqual => {
                                statement = statement.filter(crate::schema::#schema_s::#ident.ge(filter_param.compare_value.clone()));
                                count_statement = count_statement.filter(crate::schema::#schema_s::#ident.ge(filter_param.compare_value));
                            }
                            Compare::Less => {
                                statement = statement.filter(crate::schema::#schema_s::#ident.lt(filter_param.compare_value.clone()));
                                count_statement = count_statement.filter(crate::schema::#schema_s::#ident.lt(filter_param.compare_value));
                            }
                            Compare::LessAndEqual => {
                                statement = statement.filter(crate::schema::#schema_s::#ident.le(filter_param.compare_value.clone()));
                                count_statement = count_statement.filter(crate::schema::#schema_s::#ident.le(filter_param.compare_value));
                            }
                        }
                    }
        ));
    }

    let f = quote!(
        use crate::api_auth::login_impl::AuthBackend;
        use crate::controller::LOGIN_URL;
        use crate::api_doc::{default_resp_docs, empty_resp_docs};
        use crate::schema::#schema_s::dsl::#schema_s;
        use aide::axum::routing::{delete_with, get_with, post_with, put_with};
        use aide::axum::ApiRouter;
        use axum::extract::{Path};
        use diesel::r2d2::{ConnectionManager, Pool};
        use diesel::{ PgConnection};
        use crate::controller::Compare;
        use crate::controller::Filter;
        use axum_login::permission_required;
        use crate::db_models::ConnPool;
        pub fn web_routes(conn_pool: ConnPool) -> ApiRouter {
            let (router_add, router_read, router_update, router_delete) = web::get_routers();

            router_add
                .route_layer(permission_required!(AuthBackend, "common_add"))
                .merge(router_read.route_layer(permission_required!(AuthBackend, "common_read")))
                .merge(router_delete.route_layer(permission_required!(AuthBackend, "common_delete")))
                .merge(router_update.route_layer(permission_required!(AuthBackend, "common_update")))
                .with_state(conn_pool)
        }

        // web_fn_gen!(#schema, #new_model, #model);


        pub mod web {
            use crate::controller::{PageParam, PageRes};
            use super::*;
            use crate::api_doc::extractors::Json;
            use axum::extract::State;
            use diesel::r2d2::{ConnectionManager, Pool};
            use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
            use crate::api_doc::errors::AppError;

            pub fn get_routers() -> (
                ApiRouter<ConnPool>,
                ApiRouter<ConnPool>,
                ApiRouter<ConnPool>,
                ApiRouter<ConnPool>,
            ) {
            let router_add = ApiRouter::new().api_route(
                concat!("/",stringify!(#create)),
                post_with(web::create_entity, empty_resp_docs),
            );
            let router_read = ApiRouter::new()
                .api_route(
                    concat!("/",stringify!(#get)),
                    get_with(
                        web::get_entity_by_id,
                        default_resp_docs::<#model>,
                    ),
                )
                .api_route(
                    concat!("/",stringify!(#page)),
                    post_with(web::get_entity_page, empty_resp_docs),
                );
            let router_update = ApiRouter::new().api_route(
                concat!("/",stringify!(#update)),
                put_with(
                    web::update_entity_by_id,
                    default_resp_docs::<#model>,
                ),
            );
            let router_delete = ApiRouter::new().api_route(
                concat!("/",stringify!(#delete)),
                delete_with(
                    web::delete_entity_by_id,
                    default_resp_docs::<#model>,
                ),
            );
            (
                router_add,
                router_read,
                router_update,
                router_delete,
            )
            }

            pub async fn create_entity(
                State(pool): State<ConnPool>,
                Json(new_entity): Json<#new_model>,
            ) -> Result<Json<#model>, AppError> {
                let mut connection = pool.get()?;

                let result = diesel::insert_into(#schema_s)
                    .values(new_entity)
                    .returning(#model::as_returning())
                    .get_result(&mut connection)?;

                Ok(Json(result))
            }

            pub async fn update_entity_by_id(
                State(pool): State<ConnPool>,
                Path(id_param): Path<i64>,
                Json(new): Json<#new_model>,
            ) -> Result<Json<#model>, AppError> {
                let mut connection = pool.get()?;
                let result = diesel::update(#schema_s.find(id_param))
                    .set(&new)
                    .returning(#model::as_returning())
                    .get_result(&mut connection)?;
                Ok(Json(result))
            }

            pub async fn get_entity_by_id(
                State(pool): State<ConnPool>,
                Path(id_param): Path<i64>,
            ) -> Result<Json<#model>, AppError> {
                let mut connection = pool.get()?;
                let result = #schema_s
                    .find(id_param)
                    .select(#model::as_select())
                    .get_result(&mut connection)?;
                Ok(Json(result))
            }

            pub async fn delete_entity_by_id(
                State(pool): State<ConnPool>,
                Path(id_param): Path<i64>,
            ) -> Result<Json<#model>, AppError> {
                let mut connection = pool.get()?;
                let result = diesel::update(#schema_s.find(id_param))
                    .set(crate::schema::#schema_s::is_delete.eq(true))
                    .returning(#model::as_returning())
                    .get_result(&mut connection)?;
                Ok(Json(result))
            }

            pub async fn get_entity_page(
                State(pool): State<ConnPool>,
                Json(page): Json<PageParam<#builder_ident>>,
            ) -> Result<Json<PageRes<#model, #builder_ident>>, AppError> {
                let mut connection = pool.get()?;
                let off_lim = page.get_offset_limit();

                let mut statement = crate::schema::#schema_s::dsl::#schema_s.into_boxed();
                let mut count_statement = crate::schema::#schema_s::dsl::#schema_s.into_boxed();
                let filter = page.filters.clone();
                    #(#filters)*
                count_statement = count_statement.filter(crate::schema::#schema_s::is_delete.eq(false));

                let total_count = count_statement.count().get_result::<i64>(&mut connection)?;

                let res;
                let x_table = diesel_dynamic_schema::table(stringify!(#schema_s));

                let order_column = x_table.column::<diesel::sql_types::Text, _>(page.order_column.clone());
                statement = statement.filter(crate::schema::#schema_s::is_delete.eq(false));

                if page.is_desc {
                    res = statement
                        .offset(off_lim.0)
                        .limit(off_lim.1)
                        .order(order_column.desc())
                        .select(#model::as_select())
                        .load(&mut connection)?;
                } else {
                    res = statement
                        .offset(off_lim.0)
                        .limit(off_lim.1)
                        .order(order_column.asc())
                        .select(#model::as_select())
                        .load(&mut connection)?;
                }

                let page_res = PageRes::from_param_records_count(page, res,total_count);
                Ok(Json(page_res))
            }
        }

    );

    // let x_header = quote! {};

    for field in opts.fields() {
        builder.push_field(field.as_builder_field());
        // builder.push_setter_fn(field.as_setter());
        // build_fn.push_initializer(field.as_initializer());
    }

    // builder.push_build_fn();

    let mut stream = quote!(#builder);
    stream.append_all(f);
    stream.into()
}
pub trait ToSnakeCase: AsRef<str> {
    fn to_snake_case(&self) -> String;
}
impl<T> ToSnakeCase for T
where
    T: AsRef<str>,
{
    fn to_snake_case(&self) -> String {
        let text = self.as_ref();

        let mut buffer = String::with_capacity(text.len() + text.len() / 2);

        let mut text = text.chars();

        if let Some(first) = text.next() {
            let mut n2: Option<(bool, char)> = None;
            let mut n1: (bool, char) = (first.is_lowercase(), first);

            for c in text {
                let prev_n1 = n1.clone();

                let n3 = n2;
                n2 = Some(n1);
                n1 = (c.is_lowercase(), c);

                // insert underscore if acronym at beginning
                // ABc -> a_bc

                if let Some((false, c3)) = n3
                    && let Some((false, c2)) = n2
                    && n1.0
                    && c3.is_uppercase()
                    && c2.is_uppercase()
                {
                    buffer.push('_');
                }

                buffer.push_str(&prev_n1.1.to_lowercase().to_string());

                // insert underscore before next word
                // abC -> ab_c
                if let Some((true, _)) = n2
                    && n1.1.is_uppercase()
                {
                    buffer.push('_');
                }
            }

            buffer.push_str(&n1.1.to_lowercase().to_string());
        }

        buffer
    }
}
