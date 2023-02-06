;;; Directory Local Variables
;;; For more information see (info "(emacs) Directory Variables")

((rust-mode . ((eval . (setq rusty-workspace-dir "impls/rusty/Cargo.toml"))
               (eval . (setq eglot-workspace-configuration `(:rust-analyzer (:linkedProjects [,rusty-workspace-dir]) :checkOnSave (:command "clippy")))))))
