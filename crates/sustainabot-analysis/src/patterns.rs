// SPDX-License-Identifier: PMPL-1.0-or-later
// SPDX-FileCopyrightText: 2025 Jonathan D.A. Jewell

//! Pattern detection for inefficient code

/// Detect problematic patterns in code
pub fn detect_patterns(_source: &str, node: &tree_sitter::Node) -> Vec<String> {
    let mut patterns = Vec::new();

    // Count nested loops
    let loop_depth = count_loop_depth(node);
    if loop_depth >= 3 {
        patterns.push("nested_loops".to_string());
    }

    // TODO: Add more pattern detectors:
    // - Busy-wait loops (while true with no sleep/await)
    // - Large allocations (Vec::with_capacity with huge size)
    // - Inefficient I/O (no buffering)
    // - String concatenation in loops

    patterns
}

fn count_loop_depth(node: &tree_sitter::Node) -> usize {
    let is_loop = matches!(
        node.kind(),
        "for_expression" | "while_expression" | "loop_expression" | "for_statement" | "while_statement"
    );

    let mut max_child_depth = 0;
    let mut cursor = node.walk();

    if cursor.goto_first_child() {
        loop {
            let child_depth = count_loop_depth(&cursor.node());
            max_child_depth = max_child_depth.max(child_depth);

            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    if is_loop {
        1 + max_child_depth
    } else {
        max_child_depth
    }
}
