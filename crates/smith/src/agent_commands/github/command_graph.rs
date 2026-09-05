// Command graph: DOT-compiled at build time, traversed at runtime.
//
// The .dot file is the single source of truth for operation relationships.
// graph_from_file! bakes it into the binary at compile time.
// At runtime, we traverse the graph to build envelope hints.

use once_cell::sync::Lazy;
use petgraph::dot::dot_parser::{DotAttrList, DotNodeWeight, ParseFromDot};
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::{Direction, Graph};
use std::collections::HashMap;

/// The command graph, loaded once from the compile-time DOT spec.
static GRAPH: Lazy<CommandGraph> = Lazy::new(|| CommandGraph::load());

/// Helper to get an attribute value from a DOT attribute list.
fn attr_val<'a>(attrs: &'a DotAttrList<'a>, key: &str) -> Option<&'a str> {
    attrs
        .elems
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| {
            // DOT parser wraps values in quotes
            let s = *v;
            s.trim_matches('"')
        })
}

pub struct CommandGraph<'a> {
    graph: Graph<DotNodeWeight<'a>, DotAttrList<'a>>,
    index: HashMap<String, NodeIndex>,
}

impl CommandGraph<'_> {
    fn load() -> Self {
        // The DOT content is compiled into the binary. We use the same string
        // that graph_from_file! validates at compile time.
        let dot_content = include_str!("commands.dot");
        let graph: Graph<DotNodeWeight, DotAttrList> =
            ParseFromDot::try_from(dot_content).expect("DOT parse failed (should be impossible — validated at compile time)");

        let mut index = HashMap::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            index.insert(node.id.to_string(), node_idx);
        }

        CommandGraph { graph, index }
    }

    /// Get the next-step hints for an operation.
    /// `op` is the DOT node ID (e.g. "issue_view", "auth_status").
    /// `vars` are template variables to substitute (e.g. {"org": "devguyio-bot-squad", "number": "42"}).
    pub fn next_hints(&self, op: &str, vars: &HashMap<&str, String>) -> Vec<String> {
        self.edges_of_kind(op, "next", vars)
    }

    /// Get the related-operation hints for an operation.
    /// Combines explicitly declared `kind="related"` edges with auto-discovered
    /// sibling operations (other children of the same parent group).
    pub fn related_hints(&self, op: &str, vars: &HashMap<&str, String>) -> Vec<String> {
        let mut hints = self.edges_of_kind(op, "related", vars);

        // Auto-discover siblings: find parent via incoming "child" edge,
        // then list parent's other children
        if let Some(&node_idx) = self.index.get(op) {
            // Find parent: who has a "child" edge pointing to us?
            for edge in self.graph.edges_directed(node_idx, Direction::Incoming) {
                let attrs = edge.weight();
                if attr_val(attrs, "kind") == Some("child") {
                    let parent_idx = edge.source();
                    // List all other children of this parent
                    for sibling_edge in self.graph.edges_directed(parent_idx, Direction::Outgoing) {
                        let sib_attrs = sibling_edge.weight();
                        if attr_val(sib_attrs, "kind") != Some("child") {
                            continue;
                        }
                        let sib_idx = sibling_edge.target();
                        if sib_idx == node_idx {
                            continue; // skip self
                        }
                        if let Some(template) = attr_val(sib_attrs, "template") {
                            let rendered = render_template(template, vars);
                            if !hints.contains(&rendered) {
                                hints.push(rendered);
                            }
                        }
                    }
                    break; // only one parent
                }
            }
        }
        hints
    }

    /// Get recovery hints for a failed operation.
    pub fn recovery_hints(&self, op: &str, vars: &HashMap<&str, String>) -> Vec<String> {
        self.edges_of_kind(op, "recovery", vars)
    }

    fn edges_of_kind(&self, op: &str, kind: &str, vars: &HashMap<&str, String>) -> Vec<String> {
        let node_idx = match self.index.get(op) {
            Some(idx) => *idx,
            None => return vec![],
        };

        let mut hints = Vec::new();
        for edge in self.graph.edges_directed(node_idx, Direction::Outgoing) {
            let attrs = edge.weight();
            let edge_kind = attr_val(attrs, "kind").unwrap_or("");
            if edge_kind != kind {
                continue;
            }
            if let Some(template) = attr_val(attrs, "template") {
                let rendered = render_template(template, vars);
                hints.push(rendered);
            }
        }
        hints
    }
}

/// Substitute {var} placeholders in a template string.
fn render_template(template: &str, vars: &HashMap<&str, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in vars {
        result = result.replace(&format!("{{{}}}", key), value);
    }
    result
}

/// Public API: get next/related hints for an operation.
pub fn hints_for(op: &str, vars: &HashMap<&str, String>) -> (Vec<String>, Vec<String>) {
    let next = GRAPH.next_hints(op, vars);
    let related = GRAPH.related_hints(op, vars);
    (next, related)
}

/// Public API: get recovery hints for a failed operation.
pub fn recovery_for(op: &str, vars: &HashMap<&str, String>) -> Vec<String> {
    GRAPH.recovery_hints(op, vars)
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::dot::dot_parser::{DotAttrList, DotNodeWeight, ParseFromDot};

    const SAMPLE_GRAPH: &str = r#"digraph smith_agent {
        auth_status [label="auth-status" summary="Authenticated as GitHub App"]
        board [label="board" summary="Project board view"]
        issue_create [label="issue create" summary="Created issue"]
        issue_view [label="issue view" summary="Issue details"]
        issue_close [label="issue close" summary="Closed issue"]
        issue_reopen [label="issue reopen" summary="Reopened issue"]
        status_set [label="status set" summary="Transitioned status"]
        sub_issue_create [label="sub-issue create" summary="Created sub-issue"]

        auth_status -> board [kind="next"]
        board -> issue_view [kind="next"]
        board -> issue_create [kind="next"]
        issue_create -> issue_view [kind="next"]
        issue_create -> status_set [kind="next"]
        issue_create -> sub_issue_create [kind="next" condition="kind_equals_epic"]
        issue_view -> status_set [kind="next"]
        issue_view -> issue_close [kind="next" condition="state_equals_OPEN"]
        issue_view -> issue_reopen [kind="next" condition="state_equals_CLOSED"]
        issue_close -> issue_reopen [kind="next"]
        issue_reopen -> status_set [kind="next"]
        status_set -> issue_view [kind="next"]

        issue_create -> board [kind="related"]
        issue_close -> board [kind="related"]
        status_set -> board [kind="related"]

        status_set -> board [kind="recovery" failure="status_not_found"]
    }"#;

    fn parse_graph<'a>(dot: &'a str) -> Graph<DotNodeWeight<'a>, DotAttrList<'a>> {
        ParseFromDot::try_from(dot).expect("DOT parse failed")
    }

    #[test]
    fn parse_command_graph_from_dot() {
        let result: Result<Graph<DotNodeWeight, DotAttrList>, _> =
            ParseFromDot::try_from(SAMPLE_GRAPH);
        assert!(result.is_ok(), "DOT parse failed: {:?}", result.err());
    }

    #[test]
    fn dot_graph_has_expected_node_count() {
        let graph = parse_graph(SAMPLE_GRAPH);
        assert_eq!(graph.node_count(), 8, "Expected 8 operation nodes");
    }

    #[test]
    fn dot_graph_has_expected_edge_count() {
        let graph = parse_graph(SAMPLE_GRAPH);
        // 12 next + 3 related + 1 recovery = 16
        assert_eq!(graph.edge_count(), 16, "Expected 16 edges");
    }
    #[test]
    fn dot_graph_preserves_node_attributes() {
        let graph = parse_graph(SAMPLE_GRAPH);

        let has_summary = graph.node_weights().any(|node| {
            node.attr
                .elems
                .iter()
                .any(|(k, v)| *k == "summary" && v.contains("Authenticated"))
        });
        assert!(
            has_summary,
            "auth_status node should have summary attribute"
        );
    }

    #[test]
    fn dot_graph_preserves_edge_kinds() {
        let graph = parse_graph(SAMPLE_GRAPH);

        let next_count = graph
            .edge_weights()
            .filter(|e| e.elems.iter().any(|(k, v)| *k == "kind" && v.contains("next")))
            .count();
        let related_count = graph
            .edge_weights()
            .filter(|e| {
                e.elems
                    .iter()
                    .any(|(k, v)| *k == "kind" && v.contains("related"))
            })
            .count();
        let recovery_count = graph
            .edge_weights()
            .filter(|e| {
                e.elems
                    .iter()
                    .any(|(k, v)| *k == "kind" && v.contains("recovery"))
            })
            .count();

        assert_eq!(next_count, 12, "Expected 12 'next' edges");
        assert_eq!(related_count, 3, "Expected 3 'related' edges");
        assert_eq!(recovery_count, 1, "Expected 1 'recovery' edge");
    }

    #[test]
    fn dot_graph_conditional_edges() {
        let graph = parse_graph(SAMPLE_GRAPH);

        let conditional_count = graph
            .edge_weights()
            .filter(|e| e.elems.iter().any(|(k, _)| *k == "condition"))
            .count();

        assert_eq!(
            conditional_count, 3,
            "Expected 3 conditional edges (epic, OPEN, CLOSED)"
        );
    }

    #[test]
    fn dot_graph_recovery_has_failure_class() {
        let graph = parse_graph(SAMPLE_GRAPH);

        let recovery_edges: Vec<_> = graph
            .edge_weights()
            .filter(|e| e.elems.iter().any(|(k, v)| *k == "kind" && v.contains("recovery")))
            .collect();

        assert_eq!(recovery_edges.len(), 1);
        let has_failure = recovery_edges[0]
            .elems
            .iter()
            .any(|(k, v)| *k == "failure" && v.contains("status_not_found"));
        assert!(
            has_failure,
            "Recovery edge must have failure='status_not_found'"
        );
    }

    #[test]
    fn compile_time_dot_file_import() {
        // graph_from_file! parses at compile time — invalid DOT = build failure.
        let graph: Graph<DotNodeWeight, DotAttrList> = petgraph::graph_from_file!(
            "crates/smith/src/agent_commands/github/commands.dot"
        );

        assert!(
            graph.node_count() >= 33,
            "Expected 33+ nodes (26 leaves + 7 groups), got {}",
            graph.node_count()
        );
        assert!(
            graph.edge_count() >= 115,
            "Expected 115+ edges, got {}",
            graph.edge_count()
        );
    }

    #[test]
    fn runtime_hints_for_issue_view() {
        // Test that the full commands.dot can be traversed at runtime
        let vars: HashMap<&str, String> = [
            ("org", "test-org".to_string()),
            ("number", "42".to_string()),
        ]
        .into_iter()
        .collect();

        let (next, related) = super::hints_for("issue_view", &vars);
        assert!(!next.is_empty(), "issue_view should have next hints");
        // Check that templates were rendered
        for hint in &next {
            assert!(
                hint.contains("test-org"),
                "Hint should contain org: {}",
                hint
            );
        }
    }
}
