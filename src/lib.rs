use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{
            JSXAttr, JSXAttrName, JSXAttrOrSpread, JSXAttrValue, JSXOpeningElement, Lit, Program,
            Str,
        },
        transforms::base::perf::Parallel,
        utils::quote_ident,
        visit::{as_folder, noop_visit_mut_type, FoldWith, VisitMut},
    },
};

#[cfg(test)]
mod tests;

#[derive(Clone, Copy)]
struct AutoAddTestId {}

// execute parallelly
impl Parallel for AutoAddTestId {
    fn create(&self) -> Self {
        *self
    }

    fn merge(&mut self, _: Self) {}
}

impl VisitMut for AutoAddTestId {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
    noop_visit_mut_type!();

    fn visit_mut_jsx_opening_element(&mut self, element: &mut JSXOpeningElement) {
        element.attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
            span: DUMMY_SP,
            name: JSXAttrName::Ident(quote_ident!("data-testid")),
            value: Some(JSXAttrValue::Lit(Lit::Str(Str {
                span: DUMMY_SP,
                value: "first_attempt".into(),
                raw: None,
            }))),
        }))
    }
}

/// An example plugin function with macro support.
/// `plugin_transform` macro interop pointers into deserialized structs, as well
/// as returning ptr back to host.
///
/// It is possible to opt out from macro by writing transform fn manually
/// if plugin need to handle low-level ptr directly via
/// `__transform_plugin_process_impl(
///     ast_ptr: *const u8, ast_ptr_len: i32,
///     unresolved_mark: u32, should_enable_comments_proxy: i32) ->
///     i32 /*  0 for success, fail otherwise.
///             Note this is only for internal pointer interop result,
///             not actual transform result */`
///
/// This requires manual handling of serialization / deserialization from ptrs.
/// Refer swc_plugin_macro to see how does it work internally.
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(AutoAddTestId {}))
}
