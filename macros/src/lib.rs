#![deny(warnings)]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate core;
extern crate proc_macro2;
#[macro_use]
extern crate syn;

use proc_macro2::Span;
use syn::{
    parse, spanned::Spanned, AttributeArgs, FnArg, ItemFn, PathArguments, ReturnType,
    Type, Visibility,
};

use proc_macro::TokenStream;

/// Attribute to declare the entry point of the program
///
/// **IMPORTANT**: This attribute must appear exactly *once* in the dependency graph. Also, if you
/// are using Rust 1.30 the attribute must be used on a reachable item (i.e. there must be no
/// private modules between the item and the root of the crate); if the item is in the root of the
/// crate you'll be fine. This reachability restriction doesn't apply to Rust 1.31 and newer releases.
///
/// The specified function will be called by the reset handler *after* RAM has been initialized.
/// If present, the FPU will also be enabled before the function is called.
///
/// The type of the specified function must be `[unsafe] fn() -> !` (never ending function)
///
/// # Properties
///
/// The entry point will be called by the reset handler. The program can't reference to the entry
/// point, much less invoke it.
///
/// # Examples
///
/// - Simple entry point
///
/// ``` no_run
/// # #![no_main]
/// # use riscv_rt_macros::entry;
/// #[entry]
/// fn main() -> ! {
///     loop {
///         /* .. */
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    // check the function arguments
    if f.sig.inputs.len() > 3 {
        return parse::Error::new(
            f.sig.inputs.last().unwrap().span(),
            "`#[entry]` function has too many arguments",
        )
        .to_compile_error()
        .into();
    }
    for arg in &f.sig.inputs {
        match arg {
            FnArg::Receiver(_) => {
                return parse::Error::new(arg.span(), "invalid argument")
                    .to_compile_error()
                    .into();
            }
            FnArg::Typed(t) => {
                if !is_simple_type(&t.ty, "usize") {
                    return parse::Error::new(t.ty.span(), "argument type must be usize")
                        .to_compile_error()
                        .into();
                }
            }
        }
    }

    // check the function signature
    let valid_signature = f.sig.constness.is_none()
        && f.sig.asyncness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            ReturnType::Default => false,
            ReturnType::Type(_, ref ty) => matches!(**ty, Type::Never(_)),
        };

    if !valid_signature {
        return parse::Error::new(
            f.span(),
            "`#[entry]` function must have signature `[unsafe] fn([arg0: usize, ...]) -> !`",
        )
        .to_compile_error()
        .into();
    }

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    // XXX should we blacklist other attributes?
    let attrs = f.attrs;
    let unsafety = f.sig.unsafety;
    let args = f.sig.inputs;
    let stmts = f.block.stmts;

    quote!(
        #[export_name = "main"]
        #(#attrs)*
        pub #unsafety fn __risc_v_rt__main(#args) -> ! {
            #(#stmts)*
        }
    )
    .into()
}

#[allow(unused)]
fn is_simple_type(ty: &Type, name: &str) -> bool {
    if let Type::Path(p) = ty {
        if p.qself.is_none() && p.path.leading_colon.is_none() && p.path.segments.len() == 1 {
            let segment = p.path.segments.first().unwrap();
            if segment.ident == name && segment.arguments == PathArguments::None {
                return true;
            }
        }
    }
    false
}

/// Attribute to mark which function will be called at the beginning of the reset handler.
///
/// **IMPORTANT**: This attribute can appear at most *once* in the dependency graph. Also, if you
/// are using Rust 1.30 the attribute must be used on a reachable item (i.e. there must be no
/// private modules between the item and the root of the crate); if the item is in the root of the
/// crate you'll be fine. This reachability restriction doesn't apply to Rust 1.31 and newer
/// releases.
///
/// The function must have the signature of `unsafe fn()`.
///
/// The function passed will be called before static variables are initialized. Any access of static
/// variables will result in undefined behavior.
///
/// # Examples
///
/// ```
/// # use riscv_rt_macros::pre_init;
/// #[pre_init]
/// unsafe fn before_main() {
///     // do something here
/// }
///
/// # fn main() {}
/// ```
#[proc_macro_attribute]
pub fn pre_init(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    // check the function signature
    let valid_signature = f.sig.constness.is_none()
        && f.sig.asyncness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.unsafety.is_some()
        && f.sig.abi.is_none()
        && f.sig.inputs.is_empty()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            ReturnType::Default => true,
            ReturnType::Type(_, ref ty) => match **ty {
                Type::Tuple(ref tuple) => tuple.elems.is_empty(),
                _ => false,
            },
        };

    if !valid_signature {
        return parse::Error::new(
            f.span(),
            "`#[pre_init]` function must have signature `unsafe fn()`",
        )
        .to_compile_error()
        .into();
    }

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    // XXX should we blacklist other attributes?
    let attrs = f.attrs;
    let ident = f.sig.ident;
    let block = f.block;

    quote!(
        #[export_name = "__pre_init"]
        #(#attrs)*
        pub unsafe fn #ident() #block
    )
    .into()
}

/// There are three ways to connect the handler function to the actual interrupt:
/// 1. use no argument, provide a linker script entry with `PROVIDE(int_<your_interrupt_number> = <your_handler_name>)`
/// 2. use a literal integer as argument. Handler is then mapped to this interrupt number.
/// 3. use an interrupt enum from the PAC crate. Handler is mapped to this interrupt.
#[proc_macro_attribute]
pub fn interrupt_handler(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as AttributeArgs);

    // at most one argument should be provided
    if args.len() > 1 {
        return parse::Error::new(
            f.span(),
            "Too many arguments: `#[interrupt(int_nr)]` attribute must have at max one argument",
        )
        .to_compile_error()
        .into();
    }

    let attrs = f.attrs;
    let ident = f.sig.ident;
    let block = f.block;
    let ident_string = ident.to_string();

    let wrapper_ident_string = {
        // check on number of arguments
        let arg = args.get(0);
        match arg {
            // an argument exists
            Some(a) =>
            match a {
                syn::NestedMeta::Lit(l) => match l {
                    // option to supply an integer. It is treated the interrupt number, wrapper named after int_<number>
                    syn::Lit::Int(i) => "int_".to_owned() + &i.to_string(),
                    default => return parse::Error::new(
                            default.span(),
                            "Wrong type: `#[interrupt(int_nr)]` attribute must provide an integer as an argument",
                        )
                        .to_compile_error()
                        .into(),
                    },
                syn::NestedMeta::Meta(m) => match m {
                    // option to supply an identifier (e.g. an Enum name) wrapper is named after identifier
                    syn::Meta::Path(p) => match p.get_ident() {
                        Some(i) => i.to_string(),
                        None => return parse::Error::new(
                            p.span(),
                            "Wrong type: `#[interrupt(identifier)]` attribute must provide a single enum value specifying an interrupt from the PAC crate",
                        )
                        .to_compile_error()
                        .into(),
                    },
                    default => return parse::Error::new(
                        default.span(),
                        "Wrong type: `#[interrupt(..)]` attribute must have either no or one argument of type Int literal or Enum identifier",
                    )
                    .to_compile_error()
                    .into(),
                    }
            }
            // no argument exist -> wrapper is named after original function
            None => ident_string.clone()
        }
    };

    // check that function has no arguments
    if f.sig.inputs.len() != 0 {
        return parse::Error::new(
            f.sig.inputs.last().unwrap().span(),
            "`#[interrupt(..)]` handler function must not have any argument",
        )
        .to_compile_error()
        .into();
    }

    // check that function does not return anything. Not returning is also an option
    let valid_ret_type = match f.sig.output {
        ReturnType::Default => true,
        ReturnType::Type(_, ref ty) => match **ty {
            Type::Tuple(ref tuple) => tuple.elems.is_empty(),
            Type::Never(_) => true,
            _ => false,
        },
    };
    if !valid_ret_type {
        return parse::Error::new(
            f.sig.output.span(),
            "`#[interrupt(..)]` handler function must not return anything",
        )
        .to_compile_error()
        .into();
    }

    let handler_ident = format_ident!("{}_handler", ident_string);
    let wrapper_ident = format_ident!("{}", wrapper_ident_string);
    let assembly_string = format!(
        ".global {wrapper_ident_string}
    {wrapper_ident_string}:
    addi sp, sp, -(4 * 32)
    sw ra, 0(sp)
    sw t0, 4(sp)
    sw t1, 8(sp)
    sw t2, 12(sp)
    sw a0, 16(sp)
    sw a1, 20(sp)
    sw a2, 24(sp)
    sw a3, 28(sp)
    sw a4, 32(sp)
    sw a5, 36(sp)
    sw a6, 40(sp)
    sw a7, 44(sp)
    sw t3, 48(sp)
    sw t4, 52(sp)
    sw t5, 56(sp)
    sw t6, 60(sp)
    csrr t0, mcause
    csrr t1, mepc
    sw t0, 64(sp)
    sw t1, 68(sp)
    csrsi mstatus, 8 /* enable global interrupts*/

    jal {handler_ident}

    csrci mstatus, 8 /* disable global interrupts*/
    lw t0, 64(sp)
    lw t1, 68(sp)
    csrw mcause, t0
    csrw mepc, t1
    lw ra, 0(sp)
    lw t0, 4(sp)
    lw t1, 8(sp)
    lw t2, 12(sp)
    lw a0, 16(sp)
    lw a1, 20(sp)
    lw a2, 24(sp)
    lw a3, 28(sp)
    lw a4, 32(sp)
    lw a5, 36(sp)
    lw a6, 40(sp)
    lw a7, 44(sp)
    lw t3, 48(sp)
    lw t4, 52(sp)
    lw t5, 56(sp)
    lw t6, 60(sp)
    addi sp, sp, (4 * 32)
    mret
    "
    );

    if cfg!(feature = "nxti") {
        quote!(
            #(#attrs)*
            #[no_mangle]
            pub unsafe fn #wrapper_ident() #block
        )
        .into()
    } else {
        quote!(
            #(#attrs)*
            #[no_mangle]
            pub unsafe fn #handler_ident() #block

            core::arch::global_asm!(#assembly_string);

        )
        .into()
    }
}
