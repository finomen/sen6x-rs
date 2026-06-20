use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::collections::HashMap;
use stringcase::snake_case;
use syn::{Data, DeriveInput, LitInt, parse_macro_input};

#[proc_macro_derive(SenRead)]
pub fn sen_read_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let size_const = Ident::new(
        (name.to_string().to_uppercase() + "_SIZE").as_str(),
        name.span(),
    );
    let out = match input.data {
        Data::Struct(s) => {
            let fields: Vec<Ident> = s
                .fields
                .into_iter()
                .map(|field| field.ident.unwrap())
                .collect();
            quote! {
                const #size_const : usize = (#(crate::io::get_size(|x : #name| x.#fields)) + *) * 3 / 2;
                impl crate::io::FromBytes<#size_const, #name> for #name {
                    fn from_bytes_with_crc<E>(bytes: &[u8; #size_const]) -> Result<#name, crate::errors::Error<E>> {
                        let mut pos = 0;

                        fn from_bytes_with_crc<E, O, F, const N: usize>(_: fn (x: O) -> F, buf: &[u8; #size_const], offset: &mut usize ) -> Result<F, crate::errors::Error<E>> where F: crate::io::FromBytes<N, F>{
                            let res = F::from_bytes_with_crc::<E>(&crate::io::check_crc::<N, E>(&buf[*offset..(*offset + N + 1)])?);
                            *offset += N + 1;
                            res
                        }

                        Ok(#name {
                            #(
                            #fields : from_bytes_with_crc(|x : #name| x.#fields, &bytes, &mut pos)?,
                            )*
                        })
                    }
                }
                impl crate::io::ToBytes<#size_const> for #name {
                    fn to_bytes(&self) -> [u8; #size_const] {
                        let mut pos = 0;
                        let mut res = [0u8; #size_const];
                        fn to_bytes<T, const N: usize>(v : &T, buf: &mut [u8; #size_const], offset: &mut usize ) where T: crate::io::ToBytes<N>{
                            let without_crc = v.to_bytes();
                            for i in 0..N / 2 {
                                buf[*offset + i * 3] = without_crc[i * 2];
                                buf[*offset + i * 3 + 1] = without_crc[i * 2 + 1];
                                buf[*offset + i * 3 + 2] = sensirion_i2c::crc8::calculate(&without_crc[i * 2..i * 2 + 2]);
                            }
                            *offset += N * 3 / 2;
                        }
                        #(to_bytes(&self.#fields, &mut res, &mut pos);)*
                        res
                    }
                }
            }
        }
        _ => panic!("Only structs are supported"),
    };
    out.into()
}

#[proc_macro_derive(
    SenCmd,
    attributes(execution_time, read, write, send, target, alias, fetch)
)]
pub fn sen_cmd(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let out = match input.data {
        Data::Enum(s) => {
            struct TraitData {
                headers: Vec<proc_macro2::TokenStream>,
                async_headers: Vec<proc_macro2::TokenStream>,
                impls: Vec<proc_macro2::TokenStream>,
                async_impls: Vec<proc_macro2::TokenStream>,
            }

            impl TraitData {
                fn new() -> TraitData {
                    TraitData {
                        headers: Vec::new(),
                        async_headers: Vec::new(),
                        impls: Vec::new(),
                        async_impls: Vec::new(),
                    }
                }
            }

            let all_models = ["Sen62", "Sen63c", "Sen65", "Sen66", "Sen68", "Sen69c"];
            let mut impls = HashMap::new();
            for model in all_models.iter() {
                impls.insert(model.to_string(), TraitData::new());
            }

            for v in s.variants.iter() {
                let mut execution_time_opt: Option<u16> = None;
                let mut send_allowed_modes: Vec<Ident> = Vec::new();
                let mut rx_allowed_modes: Vec<Ident> = Vec::new();
                let mut tx_allowed_modes: Vec<Ident> = Vec::new();
                let mut targets: Vec<Ident> = Vec::new();
                let mut new_state: Option<Ident> = None;
                let mut send: bool = false;
                let mut read: bool = false;
                let mut write: bool = false;
                let mut fetch: bool = false;
                let mut rx: Option<Ident> = None;
                let mut tx: Option<Ident> = None;
                let mut alias: Option<Ident> = None;

                let mut docs: Vec<&syn::Attribute> = Vec::new();

                let mut send_docs: Vec<&syn::Attribute> = Vec::new();
                let mut read_docs: Vec<&syn::Attribute> = Vec::new();
                let mut write_docs: Vec<&syn::Attribute> = Vec::new();
                let mut fetch_docs: Vec<&syn::Attribute> = Vec::new();

                for attr in v.attrs.iter() {
                    if attr.path().is_ident("doc") {
                        docs.push(attr);
                    } else if attr.path().is_ident("send") {
                        send = true;
                        send_docs = docs;
                        docs = Vec::new();
                        attr.parse_nested_meta(|meta| {
                            if meta.path.is_ident("allowed_in") {
                                let value = meta.value()?;
                                let content;
                                syn::bracketed!(content in value);
                                let modes = content
                                    .parse_terminated(|p| p.parse::<Ident>(), syn::Token![,])?;
                                send_allowed_modes.extend(modes);
                                return Ok(());
                            } else if meta.path.is_ident("new_state") {
                                let lit: Ident = meta.value()?.parse()?;
                                new_state = Some(lit);
                                return Ok(());
                            }
                            Err(meta.error("unsupported cmd argument"))
                        })
                        .unwrap();
                    } else if attr.path().is_ident("read") {
                        read = true;
                        read_docs = docs;
                        docs = Vec::new();
                        attr.parse_nested_meta(|meta| {
                            if meta.path.is_ident("rx") {
                                let lit: Ident = meta.value()?.parse()?;
                                rx = Some(lit);
                                return Ok(());
                            } else if meta.path.is_ident("allowed_in") {
                                let value = meta.value()?;
                                let content;
                                syn::bracketed!(content in value);
                                let modes = content
                                    .parse_terminated(|p| p.parse::<Ident>(), syn::Token![,])?;
                                rx_allowed_modes.extend(modes);
                                return Ok(());
                            }
                            Err(meta.error("unsupported cmd argument"))
                        })
                        .unwrap();
                    } else if attr.path().is_ident("write") {
                        write = true;
                        write_docs = docs;
                        docs = Vec::new();
                        attr.parse_nested_meta(|meta| {
                            if meta.path.is_ident("tx") {
                                let lit: Ident = meta.value()?.parse()?;
                                tx = Some(lit);
                                return Ok(());
                            } else if meta.path.is_ident("allowed_in") {
                                let value = meta.value()?;
                                let content;
                                syn::bracketed!(content in value);
                                let modes = content
                                    .parse_terminated(|p| p.parse::<Ident>(), syn::Token![,])?;
                                tx_allowed_modes.extend(modes);
                                return Ok(());
                            }
                            Err(meta.error("unsupported cmd argument"))
                        })
                        .unwrap();
                    } else if attr.path().is_ident("fetch") {
                        fetch = true;
                        fetch_docs = docs;
                        docs = Vec::new();
                        attr.parse_nested_meta(|meta| {
                            if meta.path.is_ident("rx") {
                                let lit: Ident = meta.value()?.parse()?;
                                rx = Some(lit);
                                return Ok(());
                            } else if meta.path.is_ident("tx") {
                                let lit: Ident = meta.value()?.parse()?;
                                tx = Some(lit);
                                return Ok(());
                            } else if meta.path.is_ident("allowed_in") {
                                let value = meta.value()?;
                                let content;
                                syn::bracketed!(content in value);
                                let modes = content
                                    .parse_terminated(|p| p.parse::<Ident>(), syn::Token![,])?;
                                tx_allowed_modes.extend(modes);
                                return Ok(());
                            }
                            Err(meta.error("unsupported cmd argument"))
                        })
                        .unwrap();
                    } else if attr.path().is_ident("execution_time") {
                        let lit: LitInt = attr.parse_args().unwrap();
                        execution_time_opt = Some(lit.base10_parse().unwrap());
                    } else if attr.path().is_ident("target") {
                        attr.parse_nested_meta(|meta| {
                            if let Some(ident) = meta.path.get_ident() {
                                targets.push(ident.clone());
                                return Ok(());
                            }
                            Err(meta.error("unsupported cmd argument"))
                        })
                        .unwrap();
                    } else if attr.path().is_ident("alias") {
                        attr.parse_nested_meta(|meta| {
                            if let Some(ident) = meta.path.get_ident() {
                                alias = Some(ident.clone());
                                return Ok(());
                            }
                            Err(meta.error("unsupported cmd argument"))
                        })
                        .unwrap();
                    }
                }

                if send && (fetch || read || write) {
                    panic!("send could not be specified together with read, write  of fetch");
                }

                if fetch && (read || write) {
                    panic!("fetch could not be specified together with read or write");
                }

                if fetch && (tx.is_none() || rx.is_none()) {
                    panic!("fetch requires tx and rx");
                }

                if write && tx.is_none() {
                    panic!("write requires tx");
                }

                if read && rx.is_none() {
                    panic!("read requires rx");
                }

                let execution_time = match execution_time_opt {
                    Some(n) => n,
                    None => panic!("No execution time specified"),
                };

                let c_ident = alias.unwrap_or(v.ident.clone());

                if send {
                    let allowed_modes_part =
                        quote! { [#( crate::connection::State::#send_allowed_modes ),*] };
                    let command_id = &v.ident;
                    let name = Ident::new(
                        &snake_case(c_ident.to_string().as_str()),
                        Span::call_site(),
                    );

                    let update_state = match new_state {
                        Some(s) => quote! {
                            self.update_state(crate::connection::State::#s);
                        },
                        None => quote! {},
                    };

                    let ctargets: Vec<String> = if targets.is_empty() {
                        all_models.iter().map(|t| t.to_string()).collect()
                    } else {
                        targets.iter().map(|t| t.to_string()).collect()
                    };

                    let errors_doc = quote! {
                        #[doc = ""]
                        #[doc = "# Errors"]
                        #[doc = "Returns [`Error::NotAllowed`](crate::Error::NotAllowed) if the command is not allowed in the current sensor state, or [`Error::I2c`](crate::Error::I2c) if the I²C transfer fails."]
                    };
                    for tname in ctargets {
                        let t = impls.get_mut(&tname).unwrap();
                        t.headers.push(quote! {
                            #(#send_docs)*
                            #errors_doc
                            fn #name(&mut self) -> Result<(), crate::errors::Error<E>>;
                        });
                        t.async_headers.push(quote! {
                            #(#send_docs)*
                            #errors_doc
                            async fn #name(&mut self) -> Result<(), crate::errors::Error<E>>;
                        });
                        t.impls.push(quote! {
                            #(#send_docs)*
                            fn #name(&mut self) -> Result<(), crate::errors::Error<E>> {
                                self.send(crate::commands::CommandId::#command_id,#execution_time, &#allowed_modes_part)?;
                                #update_state
                                Ok(())
                            }
                        });
                        t.async_impls.push(quote! {
                            #(#send_docs)*
                            async fn #name(&mut self) -> Result<(), crate::errors::Error<E>> {
                                self.send(crate::commands::CommandId::#command_id,#execution_time, &#allowed_modes_part).await?;
                                #update_state
                                Ok(())
                            }
                        });
                    }
                }

                if read {
                    let allowed_modes_part =
                        quote! { [#( crate::connection::State::#rx_allowed_modes ),*] };
                    let command_id = &v.ident;
                    let name = Ident::new(
                        &snake_case(c_ident.to_string().as_str()),
                        Span::call_site(),
                    );

                    let ctargets: Vec<String> = if targets.is_empty() {
                        all_models.iter().map(|t| t.to_string()).collect()
                    } else {
                        targets.iter().map(|t| t.to_string()).collect()
                    };

                    let errors_doc = quote! {
                        #[doc = ""]
                        #[doc = "# Errors"]
                        #[doc = "Returns [`Error::NotAllowed`](crate::Error::NotAllowed) if the command is not allowed in the current sensor state, [`Error::I2c`](crate::Error::I2c) if the I²C transfer fails, [`Error::Crc`](crate::Error::Crc) if the response CRC is invalid, or [`Error::InvalidValue`](crate::Error::InvalidValue) if the sensor returns an out-of-range value."]
                    };
                    for tname in ctargets {
                        let t = impls.get_mut(&tname).unwrap();
                        t.headers.push(quote! {
                            #(#read_docs)*
                            #errors_doc
                            fn #name(&mut self) -> Result<#rx, crate::errors::Error<E>>;
                        });
                        t.async_headers.push(quote! {
                            #(#read_docs)*
                            #errors_doc
                            async fn #name(&mut self) -> Result<#rx, crate::errors::Error<E>>;
                        });
                        t.impls.push(quote! {
                        #(#read_docs)*
                        fn #name(&mut self) -> Result<#rx, crate::errors::Error<E>> {
                            self.read(crate::commands::CommandId::#command_id,#execution_time, &#allowed_modes_part)
                        }
                    });
                        t.async_impls.push(quote! {
                        #(#read_docs)*
                        async fn #name(&mut self) -> Result<#rx, crate::errors::Error<E>> {
                            self.read(crate::commands::CommandId::#command_id,#execution_time, &#allowed_modes_part).await
                        }
                    });
                    }
                }

                if write {
                    let allowed_modes_part =
                        quote! { [#( crate::connection::State::#tx_allowed_modes ),*] };
                    let command_id = &v.ident;
                    let name = Ident::new(
                        &format!("set_{}", &*snake_case(c_ident.to_string().as_str())),
                        Span::call_site(),
                    );

                    let ctargets: Vec<String> = if targets.is_empty() {
                        all_models.iter().map(|t| t.to_string()).collect()
                    } else {
                        targets.iter().map(|t| t.to_string()).collect()
                    };

                    let errors_doc = quote! {
                        #[doc = ""]
                        #[doc = "# Errors"]
                        #[doc = "Returns [`Error::NotAllowed`](crate::Error::NotAllowed) if the command is not allowed in the current sensor state, or [`Error::I2c`](crate::Error::I2c) if the I²C transfer fails."]
                    };
                    for tname in ctargets {
                        let t = impls.get_mut(&tname).unwrap();
                        t.headers.push(quote! {
                            #(#write_docs)*
                            #errors_doc
                            fn #name(&mut self, tx: #tx) -> Result<(), crate::errors::Error<E>>;
                        });
                        t.async_headers.push(quote! {
                        #(#write_docs)*
                        #errors_doc
                        async fn #name(&mut self, tx: #tx) -> Result<(), crate::errors::Error<E>>;
                    });
                        t.impls.push(quote! {
                        #(#write_docs)*
                        fn #name(&mut self, tx: #tx) -> Result<(), crate::errors::Error<E>> {
                            self.write(crate::commands::CommandId::#command_id,#execution_time, tx, &#allowed_modes_part)
                        }
                    });
                        t.async_impls.push(quote! {
                        #(#write_docs)*
                        async fn #name(&mut self, tx: #tx) -> Result<(), crate::errors::Error<E>> {
                            self.write(crate::commands::CommandId::#command_id,#execution_time, tx, &#allowed_modes_part).await
                        }
                    });
                    }
                }

                if fetch {
                    let allowed_modes_part =
                        quote! { [#( crate::connection::State::#tx_allowed_modes ),*] };
                    let command_id = &v.ident;
                    let name = Ident::new(
                        &format!("set_{}", &*snake_case(c_ident.to_string().as_str())),
                        Span::call_site(),
                    );

                    let ctargets: Vec<String> = if targets.is_empty() {
                        all_models.iter().map(|t| t.to_string()).collect()
                    } else {
                        targets.iter().map(|t| t.to_string()).collect()
                    };

                    let errors_doc = quote! {
                        #[doc = ""]
                        #[doc = "# Errors"]
                        #[doc = "Returns [`Error::NotAllowed`](crate::Error::NotAllowed) if the command is not allowed in the current sensor state, [`Error::I2c`](crate::Error::I2c) if the I²C transfer fails, [`Error::Crc`](crate::Error::Crc) if the response CRC is invalid, or [`Error::InvalidValue`](crate::Error::InvalidValue) if the sensor returns an out-of-range value."]
                    };
                    for tname in ctargets {
                        let t = impls.get_mut(&tname).unwrap();
                        t.headers.push(quote! {
                            #(#fetch_docs)*
                            #errors_doc
                            fn #name(&mut self, tx: #tx) -> Result<#rx, crate::errors::Error<E>>;
                        });
                        t.async_headers.push(quote! {
                        #(#fetch_docs)*
                        #errors_doc
                        async fn #name(&mut self, tx: #tx) -> Result<#rx, crate::errors::Error<E>>;
                    });
                        t.impls.push(quote! {
                        #(#fetch_docs)*
                        fn #name(&mut self, tx: #tx) -> Result<#rx, crate::errors::Error<E>> {
                            self.fetch(crate::commands::CommandId::#command_id,#execution_time, tx, &#allowed_modes_part)
                        }
                    });
                        t.async_impls.push(quote! {
                        #(#fetch_docs)*
                        async fn #name(&mut self, tx: #tx) -> Result<#rx, crate::errors::Error<E>> {
                            self.fetch(crate::commands::CommandId::#command_id,#execution_time, tx, &#allowed_modes_part).await
                        }
                    });
                    }
                }
            }
            let rendered_impls : Vec<proc_macro2::TokenStream> = impls.iter().map(|tr| {
                    let c_headers = &tr.1.headers;
                    let c_impls = &tr.1.impls;
                    let c_async_headers = &tr.1.async_headers;
                    let c_async_impls = &tr.1.async_impls;

                    let model_filter = Ident::new(tr.0,Span::call_site());
                    let name = Ident::new(&format!("{}Commands", tr.0),Span::call_site());
                    let async_name = Ident::new(&format!("{}CommandsAsync", tr.0),Span::call_site());


                    quote! {
                        #[cfg(feature = "embedded-hal")]
                        pub trait #name<E> {
                            #(#c_headers)*
                        }

                        #[cfg(feature = "embedded-hal")]
                        impl <T, E> #name<E> for T where T: crate::connection::Sen6xConnection<crate::connection::#model_filter, E> {
                             #(#c_impls)*
                        }

                        #[cfg(feature = "embedded-hal-async")]
                        pub trait #async_name<E> {
                            #(#c_async_headers)*
                        }

                        #[cfg(feature = "embedded-hal-async")]
                        impl <T, E> #async_name<E> for T where T: crate::connection::hal_async::Sen6xConnection<crate::connection::#model_filter,E> {
                            #(#c_async_impls)*
                        }
                    }
                }).collect();

            quote! {
                #(#rendered_impls)*
            }
        }
        _ => panic!("Only enums are supported"),
    };
    out.into()
}
