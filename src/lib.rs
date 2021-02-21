//! Attribute macro that automatically implements a symmetric trait
//!
//! # Motivation
//! There is a class of binary operators known as symmetric operators.
//! Formally, let `F: D x D -> V` be a binary operator, `F` is symmetric if
//! `F(a, b) = F(b, a)` for any `(a, b)`.
//!
//! Such pattern arises naturally in computational geometry. For example,
//! `is_intersected(a: Shape1<Transform>, b: Shape2<Transform>) -> bool`
//! decides whether two transformed shapes intersects. Or
//! `distance(a: Shape1<Transform>, b: Shape2<Transform>) -> f32` computes
//! the distance between two transformed shapes. Both functions could be seen
//! as symmetric binary operators. More importantly, the types of the arguments
//! can be heterogeneous. We can have `point.distance(circle)` for example.
//!
//! It is very tempting to represent an operator with a trait:
//! ```no_run
//! trait Distance<Other> {
//!     fn distance(&self, other: &Other) -> f64;
//! }
//! ```
//! And given different shapes:
//! ```no_run
//! struct Point2D {
//!     x: f64,
//!     y: f64,
//! }
//!
//! struct Disk {
//!     center: Point2D,
//!     radius: f64
//! }
//! ```
//! We can have
//! ```no_run
//! # trait Distance<Other> {
//! #     fn distance(&self, other: &Other) -> f64;
//! # }
//! # struct Point2D {
//! #     x: f64,
//! #     y: f64,
//! # }
//! # struct Disk {
//! #     center: Point2D,
//! #     radius: f64
//! # }
//! impl Distance<Point2D> for Point2D {
//!     fn distance(&self, other: &Point2D) -> f64 {
//!         let dx = self.x - other.x;
//!         let dy = self.y - other.y;
//!         (dx * dx + dy * dy).sqrt()
//!     }
//! }
//!
//! impl Distance<Disk> for Point2D {
//!     fn distance(&self, other: &Disk) -> f64 {
//!         let p_diff = self.distance(&other.center);
//!         if p_diff.le(&other.radius) {
//!             0.0_f64
//!         } else {
//!             p_diff - other.radius
//!         }
//!     }
//! }
//! ```
//! It is very helpful to also have `impl Distance<Point2D> for Disk`, but we
//! cannot use generic implementation due to conflicting implementation.
//! ```compile_fail
//! # trait Distance<Other> {
//! #     fn distance(&self, other: &Other) -> f64;
//! # }
//! # struct Point2D {
//! #     x: f64,
//! #     y: f64,
//! # }
//! # struct Disk {
//! #     center: Point2D,
//! #     radius: f64
//! # }
//! # impl Distance<Point2D> for Point2D {
//! #     fn distance(&self, other: &Point2D) -> f64 {
//! #         let dx = self.x - other.x;
//! #         let dy = self.y - other.y;
//! #         (dx * dx + dy * dy).sqrt()
//! #     }
//! # }
//! # impl Distance<Disk> for Point2D {
//! #     fn distance(&self, other: &Disk) -> f64 {
//! #         let p_diff = self.distance(&other.center);
//! #         if p_diff.le(&other.radius) {
//! #             0.0_f64
//! #         } else {
//! #             p_diff - other.radius
//! #         }
//! #     }
//! # }
//! // Conflicting implementation because this generic implementation makes
//! // Disk: Distance<Point2D>, which in turn implements
//! // Distance<Disk> for Point2D again.
//! impl<T, U> Distance<U> for T
//! where
//!     U: Distance<T>, {
//!     fn distance(&self, other: &U) -> f64 {
//!         other.distance(self)
//!     }
//! }
//! ```
//! So one has to manually implement:
//! ```no_run
//! # trait Distance<Other> {
//! #     fn distance(&self, other: &Other) -> f64;
//! # }
//! # struct Point2D {
//! #     x: f64,
//! #     y: f64,
//! # }
//! # struct Disk {
//! #     center: Point2D,
//! #     radius: f64
//! # }
//! # impl Distance<Point2D> for Point2D {
//! #     fn distance(&self, other: &Point2D) -> f64 {
//! #         let dx = self.x - other.x;
//! #         let dy = self.y - other.y;
//! #         (dx * dx + dy * dy).sqrt()
//! #     }
//! # }
//! # impl Distance<Disk> for Point2D {
//! #     fn distance(&self, other: &Disk) -> f64 {
//! #         let p_diff = self.distance(&other.center);
//! #         if p_diff.le(&other.radius) {
//! #             0.0_f64
//! #         } else {
//! #             p_diff - other.radius
//! #         }
//! #     }
//! # }
//! impl Distance<Point2D> for Disk {
//!     fn distance(&self, other: &Point2D) -> f64 {
//!         other.distance(self)
//!     }
//! }
//! ```
//! This crates tries to address this problem by introducing an attribute
//! macro to automatically implement the symmetric case.
//!
//! # Note
//! There are several constraints for a trait to be deemed symmetric:
//! * The trait must be generic, with the first non-lifetime parameter being the
//!   type for the symmetry.
//!   
//!   e.g.
//!   ```no_run
//!   trait SymmetricTrait<'a, Other, MoreType> {
//!       fn operator(&self, other: &Other) -> MoreType;
//!   }
//!   trait NotSymmetricTrait<'a, MoreType, Other> {
//!       fn operator(&self, other: &Other) -> MoreType;
//!   }
//!   ```
//! * All the methods in the trait must take exactly 2 arguments, where the
//!   first argument is `self` and the other argument is of the type for the
//!   symmetry. The two arguments must have the same family in the sense that
//!   they should both or neither be reference or mutable.
//!   
//!   e.g.
//!   ```no_run
//!   # type SomeType = i32;
//!   trait SymmetricTrait<Other> {
//!       fn operator_1(&self, other: &Other) -> SomeType;
//!       fn operator_2(self, other: Other) -> SomeType;
//!       fn operator_3(&mut self, other: &mut Other) -> SomeType;
//!   }
//!   trait NotSymmetricTrait<Other> {
//!       // reference mismatch
//!       fn operator_1(&self, other: Other) -> SomeType;
//!       // mutability mismatch
//!       fn operator_2(&self, other: &mut Other) -> SomeType;
//!       // incorrect arguments order
//!       fn operator_3(other: &mut Other, this: &mut Self) -> SomeType;
//!       // incorrect number of arguments
//!       fn operator_4(&self, other: &Other, more_other: &Other) -> SomeType;
//!   }
//!   ```
//! Associated types in a trait are allowed, and they will be transformed as:
//! ```no_run
//! # struct A {}
//! # struct B {}
//! trait TraitWithType<Other> {
//!     type SomeType;
//! }
//! impl TraitWithType<B> for A {
//!     type SomeType = i32;
//! }
//! // #[symmetric] will expands to
//! impl TraitWithType<A> for B {
//!     type SomeType = <A as TraitWithType<B>>::SomeType;
//! }
//! ```
//!  
//! # Example
//! ```
//! use symm_impl::symmetric;
//!
//! trait Distance<Other> {
//!     fn distance(&self, other: &Other) -> f64;
//! }
//! struct Point2D {
//!     x: f64,
//!     y: f64,
//! }
//! struct Disk {
//!     center: Point2D,
//!     radius: f64
//! }
//! impl Distance<Point2D> for Point2D {
//!     fn distance(&self, other: &Point2D) -> f64 {
//!         let dx = self.x - other.x;
//!         let dy = self.y - other.y;
//!         (dx * dx + dy * dy).sqrt()
//!     }
//! }
//! #[symmetric]
//! impl Distance<Disk> for Point2D {
//!     fn distance(&self, other: &Disk) -> f64 {
//!         let p_diff = self.distance(&other.center);
//!         if p_diff.le(&other.radius) {
//!             0.0_f64
//!         } else {
//!             p_diff - other.radius
//!         }
//!     }
//! }
//! /* Expands to
//! impl Distance<Point2D> for Disk {
//!     #[allow(unused_mut)]
//!     #[inline]
//!     fn distance(&self, other: &Point2D) -> f64 {
//!         <Point2D as Distance<Disk>>::distance(other, self)
//!     }
//! }
//! */
//!
//! let p = Point2D { x: 5.0, y: 4.0 };
//! let c = Disk {
//!     center: Point2D { x: 1.0, y: -2.0 },
//!     radius: 3.0,
//! };
//! assert_eq!(p.distance(&c), c.distance(&p));
//! ```

use std::{iter::FromIterator, mem};

use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::quote;
use syn::{
    parse::Parser, parse_macro_input, parse_quote, spanned::Spanned, Attribute, Block, FnArg,
    GenericArgument, ImplItem, ItemImpl, Pat, PathArguments, Type,
};

/// See module-level documentation
#[proc_macro_attribute]
pub fn symmetric(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(item as ItemImpl);

    let mirrored_ast = mirror(ast.clone());

    let expanded = quote! {
        #ast

        #mirrored_ast
    };

    proc_macro::TokenStream::from(expanded)
}

/// Take a syntax tree of impl and generate the mirror implementation for a
/// symmetric trait.
fn mirror(mut ast: ItemImpl) -> TokenStream {
    if ast.trait_.is_none() {
        // not a trait implementation
        return to_compile_error(
            "#[symmetric] can only be used on trait implementation".to_string(),
            Span::call_site(),
        );
    }
    let trait_ = ast.trait_.as_mut().unwrap();
    if let Some(bang) = trait_.0 {
        // negative marker trait
        return to_compile_error(
            "#[symmetric] cannot be used on negative trait bound".to_string(),
            bang.span.clone(),
        );
    }
    // it is guaranteed that trait_.1 is a non-empty path sequence since this is a trait impl
    let original_trait = trait_.1.clone();
    let last_segment = trait_.1.segments.last_mut().unwrap();
    let trait_generics = match &mut last_segment.arguments {
        PathArguments::AngleBracketed(generics) => generics,
        _ => {
            // no generics arguments
            return to_compile_error("expected a generic trait".to_string(), trait_.1.span());
        }
    };

    // deduce the "other" type for this trait
    let other_type = trait_generics.args.iter_mut().find_map(|arg| {
        if let GenericArgument::Type(type_arg) = arg {
            Some(type_arg)
        } else {
            None
        }
    });
    if other_type.is_none() {
        // no type arguments
        return to_compile_error(
            "symmetric trait must contain at least 1 type argument".to_string(),
            trait_generics.span(),
        );
    }
    let other_type = other_type.unwrap();

    // deduce the "self" type for this trait
    let self_type = ast.self_ty.as_mut();

    // go through items inside the block
    // 1. For every associated type, make it
    //    type SomeType = <other_type as Trait>::SomeType
    // 2. For every method, make sure it is of one of the following
    //     * f(&self, other: &other_type)
    //     * f(&mut self, other: &mut other_type)
    //     * f(self, other: other_type)
    //     * f(mut self, mut other: other_type)
    //    If there are lifetime decorations, they must be the same.
    //    replace other_type with self_type
    //    replace the body with:
    //    Trait::f(other, self)
    // 3. Leave everything else intact
    for item in ast
        .items
        .iter_mut()
        .filter(|item| matches!(item, ImplItem::Method(_) | ImplItem::Type(_)))
    {
        match item {
            ImplItem::Method(method) => {
                if let Some(variadic) = &method.sig.variadic {
                    // variadic method
                    return to_compile_error(
                        "method in a symmetric trait cannot be variadic".to_string(),
                        variadic.span(),
                    );
                }

                // verify the input arguments of the method

                if method.sig.inputs.len() != 2 {
                    // wrong number of arguments
                    return to_compile_error(
                        "expected 2 arguments".to_string(),
                        method.sig.inputs.span(),
                    );
                }

                let mut iter = method.sig.inputs.iter_mut();
                let self_arg = iter.next().unwrap();
                let other_arg = iter.next().unwrap();

                // self_arg must be one of the 4 form
                let self_arg = match self_arg {
                    FnArg::Receiver(receiver) => receiver,
                    _ => {
                        return to_compile_error(
                            "expected a receiver".to_string(),
                            self_arg.span(),
                        );
                    }
                };

                let other_arg = match other_arg {
                    FnArg::Typed(typed_arg) => typed_arg,
                    FnArg::Receiver(_) => unreachable!(),
                };

                let other_ident = if let Some((_, lifetime)) = &self_arg.reference {
                    // both should be reference with the same lifetime
                    match other_arg.ty.as_mut() {
                        Type::Reference(reference) => {
                            if self_arg.mutability != reference.mutability {
                                return to_compile_error(
                                    "mismatched mutability".to_string(),
                                    other_arg.span(),
                                );
                            }
                            if lifetime != &reference.lifetime {
                                return to_compile_error(
                                    "mismatched lifetime".to_string(),
                                    other_arg.span(),
                                );
                            }
                            // replace the underlying type for other_arg
                            reference.elem = Box::new(self_type.clone());

                            match other_arg.pat.as_ref() {
                                Pat::Ident(ident) => &ident.ident,
                                _ => {
                                    return to_compile_error(
                                        "expected an ident".to_string(),
                                        other_arg.pat.span(),
                                    );
                                }
                            }
                        }
                        _ => {
                            return to_compile_error(
                                "expected a reference".to_string(),
                                other_arg.span(),
                            );
                        }
                    }
                } else {
                    // both should be concrete
                    match other_arg.pat.as_ref() {
                        Pat::Ident(ident) => {
                            if self_arg.mutability != ident.mutability {
                                return to_compile_error(
                                    "mismatched mutability".to_string(),
                                    ident.span(),
                                );
                            }
                            // replace the type of other_arg
                            other_arg.ty = Box::new(self_type.clone());
                            &ident.ident
                        }
                        _ => {
                            return to_compile_error(
                                "expected an ident".to_string(),
                                other_arg.pat.span(),
                            );
                        }
                    }
                };

                // replace method body
                let method_name = &method.sig.ident;
                let new_block: Block = parse_quote! {
                    {
                        <#self_type as #original_trait>::#method_name(#other_ident, self)
                    }
                };
                method.block = new_block;
                method.attrs.append(
                    &mut Attribute::parse_outer
                        .parse_str("#[allow(unused_mut)]")
                        .unwrap(),
                );
                method
                    .attrs
                    .append(&mut Attribute::parse_outer.parse_str("#[inline]").unwrap());
            }
            ImplItem::Type(associated_type) => {
                // replace associated type
                let type_ident = &associated_type.ident;
                let delegated_type: Type = parse_quote! {
                    <#self_type as #original_trait>::#type_ident
                };
                associated_type.ty = delegated_type;
            }
            _ => unreachable!(),
        }
    }

    // perform swapping of the types on impl
    mem::swap(other_type, self_type);

    quote! {
        #ast
    }
}

fn to_compile_error(message: String, span: Span) -> TokenStream {
    TokenStream::from_iter(vec![
        TokenTree::Ident(Ident::new("compile_error", span)),
        TokenTree::Punct({
            let mut punct = Punct::new('!', Spacing::Alone);
            punct.set_span(span);
            punct
        }),
        TokenTree::Group({
            let mut group = Group::new(Delimiter::Brace, {
                TokenStream::from_iter(vec![TokenTree::Literal({
                    let mut string = Literal::string(&message);
                    string.set_span(span);
                    string
                })])
            });
            group.set_span(span);
            group
        }),
    ])
}
