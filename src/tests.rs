use swc_core::ecma::{
    parser::{EsConfig, Syntax},
    transforms::testing::test,
    visit::as_folder,
};

use super::AutoAddTestId;

test!(
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    }),
    |_| as_folder(AutoAddTestId {}),
    first,
    r#"var x = <div>test</div>"#,
    r#"var x = <div data-testid="first_attempt">test</div>;"#
);
