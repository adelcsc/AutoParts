
use itertools::Itertools;
use proc_macro::TokenStream;
use std::ops::Add;
use syn::__private::Span;
use quote::{quote, TokenStreamExt, ToTokens};
use syn::{Attribute, Data, ItemStruct};
use std::str::FromStr;
use convert_case::Case;
use convert_case::Casing;



#[proc_macro_derive(FilterQueryBuilder,attributes(join,vec,one))]
pub fn get_query_filter(_item : TokenStream)->TokenStream {
    let ast: syn::DeriveInput = syn::parse(_item).unwrap();
    let name = ast.ident;
    let mut tokens:Vec<proc_macro2::TokenStream>=Vec::new();
    let mut tokens_sort:Vec<proc_macro2::TokenStream>=Vec::new();
    let mut tokens_ob_fields : Vec<proc_macro2::TokenStream> = Vec::new();
    if let syn::Data::Struct(info) = ast.data {
        if let syn::Fields::Named(fields) = info.fields{
            for field in fields.named {
                let field_name : syn::Ident = field.ident.unwrap();
                let field_name_condition = format!("{}_condition",field_name);
                let field_name_condition = syn::Ident::new(field_name_condition.as_str(),field_name.span());
                let field_camel=syn::Ident::new(field_name.to_string().to_case(Case::Pascal).as_str(),field_name.span());
                let field_type = field.ty;
                let field_type = quote! {#field_type}.to_string().replace(" ","");
                let field_type={
                    if field_type.starts_with("Option<"){
                        &field_type.as_str()[7..(field_type.len()-1)]
                    } else {
                        field_type.as_str()
                    }
                };
                match field_type {
                    "String" => {
                        let token= quote! {
                            let mut #field_name_condition = Condition::any();
                            for #field_name in keys.iter().filter(|x1| { match x1.#field_name {
                                    None => {false}
                                    Some(_) => {true}
                                } }).map(|x1| { x1.#field_name.as_ref().unwrap() }).unique().collect_vec() {
                                    #field_name_condition = #field_name_condition.add(Column::#field_camel.like(#field_name.as_str()));
                                }
                            condition=condition.add(#field_name_condition);
                            };
                        let tk_sort = quote! {
                            if let Some(#field_name) = &key.#field_name{
                                is_it=x.#field_name.contains(#field_name.replace("%","").as_str());
                            }
                        };
                        tokens_sort.push(tk_sort);
                        tokens.push(token);
                        }
                    "i32" => {
                        let token= quote! {
                            let mut #field_name_condition = Condition::any();
                            let #field_name = keys.iter().filter(|x1| { match x1.#field_name {
                                    None => {false}
                                    Some(_) => {true}
                                } }).map(|x1| { x1.#field_name }).unique().collect_vec();
                            if(!#field_name.is_empty()){
                                #field_name_condition = #field_name_condition.add(Column::#field_camel.is_in(#field_name));
                            }

                            condition=condition.add(#field_name_condition);
                            };
                        tokens.push(token);
                        let tk_sort = quote! {
                            if let Some(#field_name) = key.#field_name{
                                is_it=x.#field_name==#field_name;
                            }
                        };
                        tokens_sort.push(tk_sort);
                    }
                    "Paging" =>{//TODO: UNION ALL Results
                        }
                    _ => {
                        match field.attrs.as_slice().get(0) {
                            None => {panic!("Relational Objects must define their Relations")}
                            Some(attr) => {
                                let joins = attr.tokens.to_string().replace(" ","").replace("(","").replace(")","");
                                let joins = joins.split(",").collect_vec();
                                let mut tok :Vec<proc_macro2::TokenStream> =Vec::new();
                                for x in joins {
                                    let mut join = String::from(x);
                                    join=join.add("()");
                                    tok.push(proc_macro2::TokenStream::from_str(join.as_str()).unwrap());
                                }
                                let ftype=proc_macro2::TokenStream::from_str(field_type).unwrap();
                                let token= quote! {
                                    let #field_name = keys.iter().filter(|x2| {
                                        match x2.#field_name {
                                            None => { false }
                                            Some(_) => { true }
                                        }
                                    }).map(|x3| { x3.#field_name.to_owned().unwrap() }).unique().collect_vec();
                                    if !#field_name.is_empty(){
                                        #(db_query=db_query.join(JoinType::InnerJoin,#tok);
                                            )*
                                    }
                                    //TODO:CHANGE THIS LATER queryBuilderMacro
                                    db_query=#ftype::queryBuilderMacro::<T>(#field_name.as_slice(),db_query);
                                };
                                tokens.push(token);
                            }
                        }
                        match field.attrs.as_slice().get(1) {
                            None => {}
                            Some(found) => {
                                let p = &found.path;
                                let p= quote!{#p};
                                if p.to_string().contains("one"){
                                    let inc = found.tokens.to_string().replace("(","").replace(")","").to_string();
                                    let inc = inc.split(",").collect_vec();
                                    let return_type = proc_macro2::TokenStream::from_str(inc.get(0).unwrap()).unwrap();
                                    let join_key= proc_macro2::TokenStream::from_str(inc.get(1).unwrap()).unwrap();
                                    let t=quote!{
                                        async fn #field_camel(&self,ctx:&Context<'_>,like : Option<super::#return_type::ModelInput>) -> super::#return_type::Model{
                                            let loader = ctx.data_unchecked::<DataLoader<crate::SqliteLoader>>();
                                            if let Some(mut model) = like{
                                                model.id = Some(self.#join_key);
                                                match loader.load_one(model).await.unwrap().unwrap().get(0) {
                                                    None => {return super::#return_type::Model::default()}
                                                    Some(found) => {return found.to_owned()}
                                                }
                                            }
                                            loader.load_one(super::#return_type::ModelInput{id:Some(self.#join_key),..super::#return_type::ModelInput::default()}).await.unwrap().unwrap().get(0).unwrap().to_owned()
                                        }
                                    };
                                    tokens_ob_fields.push(t);
                                } else {
                                    let inc = found.tokens.to_string().replace("(","").replace(")","").to_string();
                                    let inc = inc.split(",").collect_vec();
                                    let return_type = proc_macro2::TokenStream::from_str(inc.get(0).unwrap()).unwrap();
                                    let join_key= proc_macro2::TokenStream::from_str(inc.get(1).unwrap()).unwrap();
                                    let t=quote!{
                                            async fn #field_camel(&self,ctx:&Context<'_>,mut like : Option<super::#return_type::ModelInput>) -> Vec<super::#return_type::Model>{
                                                let loader = ctx.data_unchecked::<super::DataLoader<crate::SqliteLoader>>();
                                                match like {
                                                    None => {return loader.load_one(
                                                        super::#return_type::ModelInput{#join_key:Some(self.id),..super::#return_type::ModelInput::default()}
                                                    ).await.unwrap().unwrap()}
                                                    Some(mut like) => {
                                                        like.#join_key = Some(self.id);
                                                        return loader.load_one(like).await.unwrap().unwrap();
                                                    }
                                                }
                                            }
                                    };
                                    tokens_ob_fields.push(t);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let rs = quote! {
        use async_graphql::async_trait::async_trait;
        #[ComplexObject]
        impl Model{
            #(#tokens_ob_fields)*
        }
        impl #name {
            pub fn queryBuilderMacro<T:sea_orm::EntityTrait>(keys : &[ModelInput], mut db_query: Select<T>) -> Select<T>{
                let mut condition = Condition::all();

                #(#tokens)*
                db_query.filter(condition)
            }
            fn sortResultsMacro(res:Vec<Model>,keys:&[ModelInput],mut rs:HashMap<ModelInput,Vec<Model>>) ->HashMap<ModelInput,Vec<Model>>{
                for key in keys {
                    let res_of=res.iter().filter(|x| {
                        let mut is_it=true;
                        #(#tokens_sort)*
                        is_it
                    }).map(|x1| {x1.clone()}).collect_vec();
                    rs.insert(key.to_owned(),res_of);
                    }
                    rs
                }
        }
        #[async_trait]
        impl async_graphql::dataloader::Loader<ModelInput> for crate::SqliteLoader {
            type Value = Vec<Model>;
            type Error = ();

            async fn load(&self, keys: &[ModelInput]) -> Result<HashMap<ModelInput, Self::Value>, Self::Error> {
                let mut rs : HashMap<ModelInput,Vec<Model>> = HashMap::new();
                let mut db_query =Entity::find();
                db_query=ModelInput::queryBuilderMacro::<Entity>(keys,db_query);
                println!("{}",db_query.build(DbBackend::Sqlite).to_string());
                let db_rs=db_query.all(&self.pool).await.unwrap();
                rs = ModelInput::sortResultsMacro(db_rs,keys,rs);
                Ok(rs)
            }
        }
    };
    //panic!("{}",rs.to_string());

    rs.into()
    // let f=quote! {};
    // f.into()
}