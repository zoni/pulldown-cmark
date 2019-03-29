// This file is auto-generated by the build script
// Please, do not modify it manually

use super::test_markdown_html;

#[test]
fn regression_test_1() {
    let original = r##"<details><summary>Testing 1..2..3..</summary>

This is a test of the details element.

</details>
"##;
    let expected = r##"<details><summary>Testing 1..2..3..</summary>
<p>This is a test of the details element.</p>
</details>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_2() {
    let original = r##"see the [many] [articles] [on] [QuickCheck].

[many]: https://medium.com/@jlouis666/quickcheck-advice-c357efb4e7e6
[articles]: http://www.quviq.com/products/erlang-quickcheck/
[on]: https://wiki.haskell.org/Introduction_to_QuickCheck1
[QuickCheck]: https://hackage.haskell.org/package/QuickCheck
"##;
    let expected = r##"<p>see the 
  <a href="https://medium.com/@jlouis666/quickcheck-advice-c357efb4e7e6">many</a> 
  <a href="http://www.quviq.com/products/erlang-quickcheck/">articles</a> 
  <a href="https://wiki.haskell.org/Introduction_to_QuickCheck1">on</a> 
  <a href="https://hackage.haskell.org/package/QuickCheck">QuickCheck</a>.
</p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_3() {
    let original = r##"[![debug-stub-derive on crates.io][cratesio-image]][cratesio]
[![debug-stub-derive on docs.rs][docsrs-image]][docsrs]

[cratesio-image]: https://img.shields.io/crates/v/debug_stub_derive.svg
[cratesio]: https://crates.io/crates/debug_stub_derive
[docsrs-image]: https://docs.rs/debug_stub_derive/badge.svg?version=0.3.0
[docsrs]: https://docs.rs/debug_stub_derive/0.3.0/
"##;
    let expected = r##"<p><a href="https://crates.io/crates/debug_stub_derive"><img src="https://img.shields.io/crates/v/debug_stub_derive.svg" alt="debug-stub-derive on crates.io" /></a>
<a href="https://docs.rs/debug_stub_derive/0.3.0/"><img src="https://docs.rs/debug_stub_derive/badge.svg?version=0.3.0" alt="debug-stub-derive on docs.rs" /></a></p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_4() {
    let original = r##"|  Title A  |  Title B  |
| --------- | --------- |
| Content   | Content   |

|  Title A  |  Title B  |  Title C  |  Title D  |
| --------- | --------- | --------- | ---------:|
| Content   | Content   | Conent    | Content   |
"##;
    let expected = r##"<table><thead><tr><th>Title A  </th><th>Title B  </th></tr></thead><tbody>
<tr><td>Content   </td><td>Content   </td></tr>
</tbody></table>
<table><thead><tr><th>Title A  </th><th>Title B  </th><th>Title C  </th><th align="right">Title D  </th></tr></thead><tbody>
<tr><td>Content   </td><td>Content   </td><td>Conent    </td><td align="right">Content   </td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_5() {
    let original = r##"foo§__(bar)__
"##;
    let expected = r##"<p>foo§<strong>(bar)</strong></p>
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_6() {
    let original = r##"<https://example.com> hello
"##;
    let expected = r##"<p><a href="https://example.com">https://example.com</a> hello</p>

"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_7() {
    let original = r##"[foo][bar]

<!-- foo -->
[bar]: a
"##;
    let expected = r##"<p><a href="a">foo</a></p>
<!-- foo -->
"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_8() {
    let original = r##"<!-- <dl> -->
- **foo** (u8, u8)

  make something

- **bar** (u16, u16)

  make something
"##;
    let expected = r##"<!-- <dl> -->
<ul>
<li>
<p><strong>foo</strong> (u8, u8)</p>
<p>make something</p>
</li>
<li>
<p><strong>bar</strong> (u16, u16)</p>
<p>make something</p>
</li>
</ul>

"##;

    test_markdown_html(original, expected);
}

#[test]
fn regression_test_9() {
    let original = r##"[`
i8
`](
../../../std/primitive.i8.html
)
"##;
    let expected = r##"<p><a href="../../../std/primitive.i8.html"><code>i8</code></a></p>
"##;

    test_markdown_html(original, expected);
}