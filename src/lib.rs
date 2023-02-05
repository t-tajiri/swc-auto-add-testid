use swc_core::{ecma::{
    ast::{Program, JSXOpeningElement, JSXAttrOrSpread, JSXAttr, JSXAttrName, JSXAttrValue, Lit, Str},
    transforms::{testing::test, base::perf::Parallel},
    visit::{as_folder, FoldWith, VisitMut, noop_visit_mut_type}, utils::quote_ident,
    parser::{Syntax, EsConfig}
}, common::DUMMY_SP};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};


#[derive(Clone, Copy)]
struct AutoAddTestId {
}

// execute parallelly
impl Parallel for AutoAddTestId {
    fn create(&self) -> Self {
        *self
    }

    fn merge(&mut self, _:Self) {}
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
                raw: None
            })))
        })
        )
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

// An example to test plugin transform.
// Recommended strategy to test plugin's transform is verify
// the Visitor's behavior, instead of trying to run `process_transform` with mocks
// unless explicitly required to do so.
test!(
    Syntax::Es(EsConfig { jsx: true, ..Default::default() }),
    |_| as_folder(AutoAddTestId {}),
    first,
    // Input codes
    r#"var x = <div>test</div>"#,
    // Output codes after transformed with plugin
    r#"var x = <div data-testid="first_attempt">test</div>;"#
);