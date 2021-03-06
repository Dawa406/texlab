use futures_boxed::boxed;
use texlab_protocol::RangeExt;
use texlab_protocol::*;
use texlab_syntax::*;
use texlab_workspace::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BibtexFieldHoverProvider;

impl FeatureProvider for BibtexFieldHoverProvider {
    type Params = TextDocumentPositionParams;
    type Output = Option<Hover>;

    #[boxed]
    async fn execute<'a>(
        &'a self,
        request: &'a FeatureRequest<TextDocumentPositionParams>,
    ) -> Option<Hover> {
        if let SyntaxTree::Bibtex(tree) = &request.document().tree {
            for node in tree.find(request.params.position) {
                if let BibtexNode::Field(field) = node {
                    if field.name.range().contains(request.params.position) {
                        let documentation = LANGUAGE_DATA.field_documentation(field.name.text())?;
                        return Some(Hover {
                            contents: HoverContents::Markup(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: documentation.into(),
                            }),
                            range: Some(field.name.range()),
                        });
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texlab_protocol::Position;

    #[test]
    fn known_field() {
        let hover = test_feature(
            BibtexFieldHoverProvider,
            FeatureSpec {
                files: vec![FeatureSpec::file("foo.bib", "@article{foo, author = bar}")],
                main_file: "foo.bib",
                position: Position::new(0, 15),
                ..FeatureSpec::default()
            },
        );
        assert_eq!(
            hover,
            Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: LANGUAGE_DATA.field_documentation("author").unwrap().into(),
                }),
                range: Some(Range::new_simple(0, 14, 0, 20)),
            })
        );
    }

    #[test]
    fn unknown_field() {
        let hover = test_feature(
            BibtexFieldHoverProvider,
            FeatureSpec {
                files: vec![FeatureSpec::file("foo.bib", "@article{foo, bar = baz}")],
                main_file: "foo.bib",
                position: Position::new(0, 15),
                ..FeatureSpec::default()
            },
        );
        assert_eq!(hover, None);
    }

    #[test]
    fn entry_key() {
        let hover = test_feature(
            BibtexFieldHoverProvider,
            FeatureSpec {
                files: vec![FeatureSpec::file("foo.bib", "@article{foo, bar = baz}")],
                main_file: "foo.bib",
                position: Position::new(0, 11),
                ..FeatureSpec::default()
            },
        );
        assert_eq!(hover, None);
    }

    #[test]
    fn latex() {
        let hover = test_feature(
            BibtexFieldHoverProvider,
            FeatureSpec {
                files: vec![FeatureSpec::file("foo.tex", "")],
                main_file: "foo.tex",
                position: Position::new(0, 0),
                ..FeatureSpec::default()
            },
        );
        assert_eq!(hover, None);
    }
}
