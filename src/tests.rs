use swc_core::ecma::{
    parser::{EsConfig, Syntax},
    transforms::testing::test,
    visit::{as_folder, Fold},
};

use super::AutoAddTestId;

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

fn executor() -> impl Fold {
    as_folder(AutoAddTestId {})
}

test!(
    syntax(),
    |_| executor(),
    when_single_elements_add_data_attribute,
    r#"var x = <div>test</div>"#,
    r#"var x = <div data-testid="first_attempt">test</div>;"#
);

test!(
    syntax(),
    |_| executor(),
    when_react_fragments_should_not_data_attribute,
    r#"var x = <><div>test</div></>"#,
    r#"var x = <><div data-testid="first_attempt">test</div></>;"#
);

//TODO
test!(
    ignore,
    syntax(),
    |_| executor(),
    when_nested_elements_then_add_only_first_element,
    r#"var x = <div><div>test</div></div>"#,
    r#"var x = <div data-testid="first_attempt"><div>test</div></div>"#
);
