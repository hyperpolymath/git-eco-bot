;; SPDX-License-Identifier: PMPL-1.0-or-later
;; STATE.scm - Project state for oikos
;; Media-Type: application/vnd.state+scm

(state
  (metadata
    (version "0.0.1")
    (schema-version "1.0")
    (created "2026-01-03")
    (updated "2026-01-25")
    (project "oikos")
    (repo "github.com/hyperpolymath/sustainabot"))

  (project-context
    (name "sustainabot (oikos)")
    (tagline "Ecological & Economic Code Analysis")
    (tech-stack ("Haskell" "OCaml" "Rust" "Deno" "Datalog" "Guix")))

  (current-position
    (phase "alpha-refactor")
    (overall-completion 15)
    (components 
      (("analyzers" . "active")
       ("bot-integration" . "migrating-to-rescript")
       ("policy-engine" . "de-pythonizing")))
    (working-features ("Haskell AST parsing" "Guix/Nix environments")))

  (route-to-mvp
    (milestones 
      ("0.1.0" . "Complete Python removal")
      ("0.2.0" . "Functional Carbon Scoring (SCI baseline)")
      ("0.3.0" . "GitHub Action integration")))

  (blockers-and-issues
    (critical "Python logic remains in policy-engine")
    (high "Pareto optimization rules incomplete")
    (medium "Documentation needs AsciiDoc migration"))

  (critical-next-actions
    (immediate "Port DeepProbLog logic to Haskell or Datalog")
    (this-week "Consolidate .md files to .adoc")
    (this-month "First successful PR analysis on GitHub"))

  (session-history 
    ("2026-01-25" . "Refining SCM metadata and language policy compliance")))
