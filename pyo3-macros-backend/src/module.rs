// Copyright (c) 2017-present PyO3 Project and Contributors
//! Code generation for the function that initializes a python module and adds classes and function.

use crate::{
    attributes::{
        self, is_attribute_ident, take_attributes, take_pyo3_options, CrateAttribute, NameAttribute,
    },
    pyfunction::{impl_wrap_pyfunction, PyFunctionOptions},
    utils::{get_pyo3_crate, PythonDoc},
    wrap::module_def_ident,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token::Comma,
    Ident, Path, Result, Visibility,
};

#[derive(Default)]
pub struct PyModuleOptions {
    krate: Option<CrateAttribute>,
    name: Option<syn::Ident>,
}

impl PyModuleOptions {
    pub fn from_attrs(attrs: &mut Vec<syn::Attribute>) -> Result<Self> {
        let mut options: PyModuleOptions = Default::default();

        for option in take_pyo3_options(attrs)? {
            match option {
                PyModulePyO3Option::Name(name) => options.set_name(name.value.0)?,
                PyModulePyO3Option::Crate(path) => options.set_crate(path)?,
            }
        }

        Ok(options)
    }

    fn set_name(&mut self, name: syn::Ident) -> Result<()> {
        ensure_spanned!(
            self.name.is_none(),
            name.span() => "`name` may only be specified once"
        );

        self.name = Some(name);
        Ok(())
    }

    fn set_crate(&mut self, path: CrateAttribute) -> Result<()> {
        ensure_spanned!(
            self.krate.is_none(),
            path.span() => "`crate` may only be specified once"
        );

        self.krate = Some(path);
        Ok(())
    }
}

/// Generates the function that is called by the python interpreter to initialize the native
/// module
pub fn pymodule_impl(
    fnname: &Ident,
    options: PyModuleOptions,
    doc: PythonDoc,
    visibility: &Visibility,
) -> TokenStream {
    let name = options.name.unwrap_or_else(|| fnname.unraw());
    let krate = get_pyo3_crate(&options.krate);
    let cb_name = Ident::new(&format!("PyInit_{}", name), Span::call_site());

    let module_def_name = module_def_ident(fnname);

    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        /// This autogenerated function is called by the python interpreter when importing
        /// the module.
        pub unsafe extern "C" fn #cb_name() -> *mut #krate::ffi::PyObject {
            unsafe { #module_def_name.module_init() }
        }

        #[doc(hidden)]
        #visibility static #module_def_name: #krate::impl_::pymodule::ModuleDef = unsafe {
            #krate::impl_::pymodule::ModuleDef::new(concat!(stringify!(#name), "\0"), #doc, #krate::impl_::pymodule::ModuleInitializer(#fnname))
        };
    }
}

/// Finds and takes care of the #[pyfn(...)] in `#[pymodule]`
pub fn process_functions_in_module(func: &mut syn::ItemFn) -> syn::Result<()> {
    let mut stmts: Vec<syn::Stmt> = Vec::new();

    for mut stmt in func.block.stmts.drain(..) {
        if let syn::Stmt::Item(syn::Item::Fn(func)) = &mut stmt {
            if let Some(pyfn_args) = get_pyfn_attr(&mut func.attrs)? {
                let module_name = pyfn_args.modname;
                let wrapped_function = impl_wrap_pyfunction(func, pyfn_args.options)?;
                let name = &func.sig.ident;
                let statements: Vec<syn::Stmt> = syn::parse_quote! {
                    #wrapped_function
                    #module_name.add_function(#name::wrap(#name::DEF, #module_name)?)?;
                };
                stmts.extend(statements);
            }
        };
        stmts.push(stmt);
    }

    func.block.stmts = stmts;
    Ok(())
}

pub struct PyFnArgs {
    modname: Path,
    options: PyFunctionOptions,
}

impl Parse for PyFnArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let modname = input.parse().map_err(
            |e| err_spanned!(e.span() => "expected module as first argument to #[pyfn()]"),
        )?;

        if input.is_empty() {
            return Ok(Self {
                modname,
                options: Default::default(),
            });
        }

        let _: Comma = input.parse()?;

        Ok(Self {
            modname,
            options: input.parse()?,
        })
    }
}

/// Extracts the data from the #[pyfn(...)] attribute of a function
fn get_pyfn_attr(attrs: &mut Vec<syn::Attribute>) -> syn::Result<Option<PyFnArgs>> {
    let mut pyfn_args: Option<PyFnArgs> = None;

    take_attributes(attrs, |attr| {
        if is_attribute_ident(attr, "pyfn") {
            ensure_spanned!(
                pyfn_args.is_none(),
                attr.span() => "`#[pyfn] may only be specified once"
            );
            pyfn_args = Some(attr.parse_args()?);
            Ok(true)
        } else {
            Ok(false)
        }
    })?;

    if let Some(pyfn_args) = &mut pyfn_args {
        pyfn_args
            .options
            .add_attributes(take_pyo3_options(attrs)?)?;
    }

    Ok(pyfn_args)
}

enum PyModulePyO3Option {
    Crate(CrateAttribute),
    Name(NameAttribute),
}

impl Parse for PyModulePyO3Option {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(attributes::kw::name) {
            input.parse().map(PyModulePyO3Option::Name)
        } else if lookahead.peek(syn::Token![crate]) {
            input.parse().map(PyModulePyO3Option::Crate)
        } else {
            Err(lookahead.error())
        }
    }
}
