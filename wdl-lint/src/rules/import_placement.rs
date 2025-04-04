//! A lint rule for import placements.

use wdl_ast::AstNode;
use wdl_ast::Diagnostic;
use wdl_ast::Diagnostics;
use wdl_ast::Document;
use wdl_ast::Span;
use wdl_ast::SupportedVersion;
use wdl_ast::SyntaxElement;
use wdl_ast::SyntaxKind;
use wdl_ast::VisitReason;
use wdl_ast::Visitor;
use wdl_ast::v1::ImportStatement;
use wdl_ast::v1::StructDefinition;
use wdl_ast::v1::TaskDefinition;
use wdl_ast::v1::WorkflowDefinition;

use crate::Rule;
use crate::Tag;
use crate::TagSet;

/// The identifier for the import placement rule.
const ID: &str = "ImportPlacement";

/// Creates a "misplaced import" diagnostic.
fn misplaced_import(span: Span) -> Diagnostic {
    Diagnostic::warning("misplaced import")
        .with_rule(ID)
        .with_highlight(span)
        .with_fix(
            "move this import so that it comes after the version statement but before any \
             document items",
        )
}

/// Detects incorrect import placements.
#[derive(Default, Debug, Clone, Copy)]
pub struct ImportPlacementRule {
    /// Whether or not an import statement is considered invalid.
    invalid: bool,
}

impl Rule for ImportPlacementRule {
    fn id(&self) -> &'static str {
        ID
    }

    fn description(&self) -> &'static str {
        "Ensures that imports are placed between the version statement and any document items."
    }

    fn explanation(&self) -> &'static str {
        "All import statements should follow the WDL version declaration with one empty line \
         between the version and the first import statement."
    }

    fn tags(&self) -> TagSet {
        TagSet::new(&[Tag::Clarity])
    }

    fn exceptable_nodes(&self) -> Option<&'static [wdl_ast::SyntaxKind]> {
        Some(&[
            SyntaxKind::VersionStatementNode,
            SyntaxKind::ImportStatementNode,
        ])
    }
}

impl Visitor for ImportPlacementRule {
    type State = Diagnostics;

    fn document(
        &mut self,
        _: &mut Self::State,
        reason: VisitReason,
        _: &Document,
        _: SupportedVersion,
    ) {
        if reason == VisitReason::Exit {
            return;
        }

        // Reset the visitor upon document entry
        *self = Default::default();
    }

    fn import_statement(
        &mut self,
        state: &mut Self::State,
        reason: VisitReason,
        stmt: &ImportStatement,
    ) {
        if reason == VisitReason::Exit {
            return;
        }

        if self.invalid {
            state.exceptable_add(
                misplaced_import(stmt.span()),
                SyntaxElement::from(stmt.inner().clone()),
                &self.exceptable_nodes(),
            );
        }
    }

    fn struct_definition(
        &mut self,
        _: &mut Self::State,
        reason: VisitReason,
        _: &StructDefinition,
    ) {
        if reason == VisitReason::Exit {
            return;
        }

        // Saw an item other than an import, imports are no longer valid
        self.invalid = true;
    }

    fn task_definition(&mut self, _: &mut Self::State, reason: VisitReason, _: &TaskDefinition) {
        if reason == VisitReason::Exit {
            return;
        }

        // Saw an item other than an import, imports are no longer valid
        self.invalid = true;
    }

    fn workflow_definition(
        &mut self,
        _: &mut Self::State,
        reason: VisitReason,
        _: &WorkflowDefinition,
    ) {
        if reason == VisitReason::Exit {
            return;
        }

        // Saw an item other than an import, imports are no longer valid
        self.invalid = true;
    }
}
