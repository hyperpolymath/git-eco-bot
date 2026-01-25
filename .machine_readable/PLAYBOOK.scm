;; SPDX-License-Identifier: PMPL-1.0-or-later
;; PLAYBOOK.scm - Operational runbook for oikos

(define playbook
  `((version . "1.0.0")
    (procedures
      ((deploy . (("build" . "just build")
                  ("test" . "just test")
                  ("release" . "just release")))
       (environment . (("guix" . "guix shell")
                       ("nix" . "nix develop")))
       (rollback . (("git" . "git revert HEAD")))
       (debug . (("logs" . "podman-compose logs -f")
                 ("repl" . "guile")))))
    (alerts 
      ((carbon-spike . "Triggered when SCI score drops > 20 points")
       (policy-violation . "Triggered on Python detection")))
    (contacts 
      ((maintainer . "hyperpolymath")))))
