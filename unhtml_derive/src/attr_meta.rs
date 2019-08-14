use proc_macro::{Diagnostic, Level};
use scraper::Selector;
use std::convert::TryFrom;
use syn::{Attribute, Lit, Meta, NestedMeta};

const HTML_ATTR: &str = "html";
const SELECTOR_ATTR: &str = "selector";
const ATTR_ATTR: &str = "attr";
const DEFAULT_ATTR: &str = "default";

macro_rules! diagnostic_invalid_attribute {
    ($attr:expr) => {
        Diagnostic::new(Level::Error, format!("invalid `html` attribute: {}", $attr))
    };
}

#[derive(Debug)]
pub struct AttrMeta {
    pub selector: Option<String>,
    pub attr: Option<String>,
    pub default: bool,
}

impl TryFrom<Vec<Attribute>> for AttrMeta {
    type Error = Diagnostic;
    fn try_from(attrs: Vec<Attribute>) -> Result<Self, Self::Error> {
        let meta = filter_attrs(attrs)?;
        let mut selector = None;
        let mut attr = None;
        let mut default = false;
        match meta {
            Meta::Word(ident) => (),
            Meta::NameValue(_) => return Err(diagnostic_invalid_attribute!(quote!(#meta))),
            Meta::List(list) => {
                for nested_meta in list.nested.iter() {
                    match nested_meta {
                        NestedMeta::Literal(_) => {
                            return Err(diagnostic_invalid_attribute!(quote!(#meta)))
                        }
                        NestedMeta::Meta(inner_meta) => match inner_meta {
                            Meta::Word(ident) if ident == DEFAULT_ATTR => {
                                default = true;
                            }
                            Meta::NameValue(named_value) => {
                                if named_value.ident == SELECTOR_ATTR {
                                    let selector_lit = get_lit_str_value(&named_value.lit).ok_or(
                                        Diagnostic::new(
                                            Level::Error,
                                            "selector should be str literal",
                                        ),
                                    )?;
                                    check_selector(&selector_lit)?;
                                    selector = Some(selector_lit);
                                } else if named_value.ident == ATTR_ATTR {
                                    let attr_lit = get_lit_str_value(&named_value.lit).ok_or(
                                        Diagnostic::new(Level::Error, "attr should be str literal"),
                                    )?;
                                    attr = Some(attr_lit);
                                }
                            }
                            _ => return Err(diagnostic_invalid_attribute!(quote!(#meta))),
                        },
                    }
                }
            }
        }
        Ok(Self {
            selector,
            attr,
            default,
        })
    }
}

fn filter_attrs(attrs: Vec<Attribute>) -> Result<Meta, Diagnostic> {
    let attrs: Vec<Attribute> = attrs
        .into_iter()
        .filter_map(|attr| {
            if attr.path.is_ident(HTML_ATTR) {
                Some(attr)
            } else {
                None
            }
        })
        .collect();
    if attrs.len() != 1 {
        return Err(Diagnostic::new(
            Level::Error,
            "each derived target or field can only have one `html` attribute",
        ));
    }
    attrs
        .into_iter()
        .first()
        .unwrap()
        .parse_meta()
        .map_err(|err| diagnostic_invalid_attribute!(err))
}

fn check_selector(selector: &str) -> Result<(), Diagnostic> {
    Selector::parse(selector)
        .map(|_| ())
        .map_err(|err| Diagnostic::new(Level::Error, format!("invalid css selector: {}", selector)))
}

fn get_lit_str_value(lit: &Lit) -> Option<String> {
    if let &Lit::Str(ref str_lit) = lit {
        Some(str_lit.value())
    } else {
        None
    }
}