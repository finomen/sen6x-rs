use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
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

#[proc_macro_derive(SenCmd, attributes(cmd))]
pub fn sen_cmd(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let out = match input.data {
        Data::Enum(s) => {
            const COMMAND_ATTR_NAME: &'static str = "cmd";
            const RX_PARAM_NAME: &'static str = "rx";
            const TX_PARAM_NAME: &'static str = "tx";
            const REG_PARAM_NAME: &'static str = "reg";
            const NEW_STATE_PARAM_NAME: &'static str = "new_state";
            const EXECUTION_TIME_PARAM_NAME: &'static str = "execution_time";

            let mut headers: Vec<proc_macro2::TokenStream> = Vec::new();
            let mut async_headers: Vec<proc_macro2::TokenStream> = Vec::new();

            let mut impls: Vec<proc_macro2::TokenStream> = Vec::new();
            let mut async_impls: Vec<proc_macro2::TokenStream> = Vec::new();

            for v in s.variants.iter() {
                for attr in v
                    .attrs
                    .iter()
                    .filter(|a| a.path().is_ident(COMMAND_ATTR_NAME))
                {
                    let mut execution_time_opt: Option<u16> = None;
                    let mut allowed_modes: Vec<Ident> = Vec::new();
                    let mut rx: Option<Ident> = None;
                    let mut tx: Option<Ident> = None;
                    let mut reg: Option<Ident> = None;
                    let mut new_state: Option<Ident> = None;

                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident(EXECUTION_TIME_PARAM_NAME) {
                            let lit: LitInt = meta.value()?.parse()?;
                            execution_time_opt = Some(lit.base10_parse()?);
                            return Ok(());
                        } else if meta.path.is_ident(RX_PARAM_NAME) {
                            let lit: Ident = meta.value()?.parse()?;
                            rx = Some(lit);
                            return Ok(());
                        } else if meta.path.is_ident(TX_PARAM_NAME) {
                            let lit: Ident = meta.value()?.parse()?;
                            tx = Some(lit);
                            return Ok(());
                        } else if meta.path.is_ident(REG_PARAM_NAME) {
                            let lit: Ident = meta.value()?.parse()?;
                            reg = Some(lit);
                            return Ok(());
                        } else if meta.path.is_ident(NEW_STATE_PARAM_NAME) {
                            let lit: Ident = meta.value()?.parse()?;
                            new_state = Some(lit);
                            return Ok(());
                        } else if let Some(ident) = meta.path.get_ident() {
                            allowed_modes.push(ident.clone());
                            return Ok(());
                        }
                        Err(meta.error("unsupported cmd argument"))
                    })
                    .unwrap();

                    let execution_time = match execution_time_opt {
                        Some(n) => n,
                        None => panic!("No execution time specified"),
                    };

                    let allowed_modes_part =
                        quote! { [#( crate::connection::State::#allowed_modes ),*] };
                    let command_id = &v.ident;
                    let name = Ident::new(
                        &*snake_case(v.ident.to_string().as_str()),
                        Span::call_site(),
                    );

                    let update_state = match new_state {
                        Some(s) => quote! {
                            self.update_state(crate::connection::State::#s);
                        },
                        None => quote! {},
                    };

                    match (rx, tx, reg) {
                        (None, None, None) => {
                            headers.push(quote! {
                                fn #name(&mut self) -> Result<(), crate::errors::Error<E>>;
                            });
                            async_headers.push(quote! {
                                async fn #name(&mut self) -> Result<(), crate::errors::Error<E>>;
                            });
                            impls.push(quote! {
                            fn #name(&mut self) -> Result<(), crate::errors::Error<E>> {
                                self.send(crate::commands::CommandId::#command_id,#execution_time, &#allowed_modes_part)?;
                                #update_state
                                Ok(())
                            }
                        });
                            async_impls.push(quote! {
                            async fn #name(&mut self) -> Result<(), crate::errors::Error<E>> {
                                self.send(crate::commands::CommandId::#command_id, #execution_time, &#allowed_modes_part).await?;
                                #update_state
                                Ok(())
                            }
                        });
                        }
                        (Some(rx), None, None) => {
                            headers.push(quote! {
                                fn #name(&mut self) -> Result<#rx, crate::errors::Error<E>>;
                            });
                            async_headers.push(quote! {
                                async fn #name(&mut self) -> Result<#rx, crate::errors::Error<E>>;
                            });
                            impls.push(quote! {
                            fn #name(&mut self) -> Result<#rx, crate::errors::Error<E>> {
                                self.read(crate::commands::CommandId::#command_id,#execution_time, &#allowed_modes_part)
                            }
                        });
                            async_impls.push(quote! {
                            async fn #name(&mut self) -> Result<#rx, crate::errors::Error<E>> {
                                self.read(crate::commands::CommandId::#command_id,#execution_time, &#allowed_modes_part).await
                            }
                        });
                        }
                        (None, Some(tx), None) => {
                            headers.push(quote! {
                                fn #name(&mut self, tx: #tx) -> Result<(), crate::errors::Error<E>>;
                            });
                            async_headers.push(quote! {
                            async fn #name(&mut self, tx: #tx) -> Result<(), crate::errors::Error<E>>;
                        });
                            impls.push(quote! {
                            fn #name(&mut self, tx: #tx) -> Result<(), crate::errors::Error<E>> {
                                self.write(crate::commands::CommandId::#command_id,#execution_time, tx, &#allowed_modes_part)
                            }
                        });
                            async_impls.push(quote! {
                            async fn #name(&mut self, tx: #tx) -> Result<(), crate::errors::Error<E>> {
                                self.write(crate::commands::CommandId::#command_id,#execution_time, tx, &#allowed_modes_part).await
                            }
                        });
                        }
                        (Some(rx), Some(tx), None) => {
                            headers.push(quote! {
                            fn #name(&mut self, tx: #tx) -> Result<#rx, crate::errors::Error<E>>;
                        });
                            async_headers.push(quote! {
                                async fn #name(&mut self, tx: #tx) -> Result<#rx,
                                    crate::errors::Error<E>>;
                            });
                            impls.push(quote! {
                            fn #name(&mut self, tx: #tx) -> Result<#rx, crate::errors::Error<E>> {
                                self.fetch(crate::commands::CommandId::#command_id,#execution_time, tx, &#allowed_modes_part)
                            }
                        });
                            async_impls.push(quote! {
                            async fn #name(&mut self, tx: #tx) -> Result<#rx, crate::errors::Error<E>> {
                                self.fetch(crate::commands::CommandId::#command_id,#execution_time, tx, &#allowed_modes_part).await
                            }
                        });
                        }
                        (None, None, Some(reg)) => {
                            let name_string = snake_case(v.ident.to_string().as_str());
                            let getter =
                                Ident::new(&format!("get_{}", name_string), Span::call_site());
                            let setter =
                                Ident::new(&format!("set_{}", name_string), Span::call_site());

                            headers.push(quote! {
                                fn #getter(&mut self) -> Result<#reg, crate::errors::Error<E>>;
                                fn #setter(&mut self, tx: #reg) -> Result<(), crate::errors::Error<E>>;
                            });
                            async_headers.push(quote! {
                                async fn #getter(&mut self) -> Result<#reg, crate::errors::Error<E>>;
                                async fn #setter(&mut self, tx: #reg) -> Result<(), crate::errors::Error<E>>;
                            });
                            impls.push(quote! {
                                fn #getter(&mut self) -> Result<#reg, crate::errors::Error<E>> {
                                    self.read(crate::commands::CommandId::#command_id,#execution_time, &#allowed_modes_part)
                                }
                                fn #setter(&mut self, tx: #reg) -> Result<(), crate::errors::Error<E>> {
                                    self.write(crate::commands::CommandId::#command_id,#execution_time, tx, &#allowed_modes_part)
                                }
                            });
                            async_impls.push(quote! {
                                async fn #getter(&mut self) -> Result<#reg, crate::errors::Error<E>> {
                                    self.read(crate::commands::CommandId::#command_id,#execution_time, &#allowed_modes_part).await
                                }
                                async fn #setter(&mut self, tx: #reg) -> Result<(), crate::errors::Error<E>> {
                                    self.write(crate::commands::CommandId::#command_id,#execution_time, tx, &#allowed_modes_part).await
                                }
                            });
                        }
                        _ => panic!("rx/tx and reg could noe be specified at the same time"),
                    }
                }
            }

            quote! {
                #[cfg(feature = "embedded-hal")]
                pub trait Sen6xCommands<E> {
                    #(#headers)*
                }

                #[cfg(feature = "embedded-hal")]
                impl <T, E> Sen6xCommands<E> for T where T: crate::connection::Sen6xConnection<E> {
                     #(#impls)*
                }

                #[cfg(feature = "embedded-hal-async")]
                pub trait Sen6xCommandsAsync<E> {
                    #(#async_headers)*
                }

                #[cfg(feature = "embedded-hal-async")]
                impl <T, E> Sen6xCommandsAsync<E> for T where T: crate::connection::hal_async::Sen6xConnection<E> {
                    #(#async_impls)*
                }
            }
        }
        _ => panic!("Only enums are supported"),
    };
    out.into()
}
