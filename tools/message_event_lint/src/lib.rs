use std::path::Path;
use syn::{visit::Visit, Expr, ExprMethodCall, File, ItemFn, Path as SynPath, Type, TypePath};

#[derive(Debug, PartialEq, Eq)]
pub struct Finding {
    pub file: String,
    pub fn_name: String,
}

struct FnVisitor<'a> {
    src: &'a str,
    findings: Vec<Finding>,
    current_fn: Option<String>,
    has_message_writer: bool,
    has_side_effect: bool,
}

impl<'a> FnVisitor<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            src,
            findings: Vec::new(),
            current_fn: None,
            has_message_writer: false,
            has_side_effect: false,
        }
    }

    fn check_and_reset(&mut self, span_fn_name: &str, file: &Path) {
        if self.has_message_writer && self.has_side_effect {
            self.findings.push(Finding {
                file: file.to_string_lossy().to_string(),
                fn_name: span_fn_name.to_string(),
            });
        }
        self.has_message_writer = false;
        self.has_side_effect = false;
    }
}

impl<'ast> Visit<'ast> for FnVisitor<'_> {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        // set current fn
        let fn_name = i.sig.ident.to_string();
        self.current_fn = Some(fn_name.clone());
        // inspect arguments types for MessageWriter
        for input in i.sig.inputs.iter() {
            if let syn::FnArg::Typed(pat_ty) = input {
                if matches_message_writer(&*pat_ty.ty) {
                    self.has_message_writer = true;
                }
            }
        }
        // visit body
        syn::visit::visit_block(self, &i.block);
        // after visiting, check combination
        self.check_and_reset(&fn_name, Path::new("<in-memory>"));
    }

    fn visit_type_path(&mut self, i: &'ast TypePath) {
        // detect local MessageWriter mentions in types
        if i.path.segments.iter().any(|s| s.ident == "MessageWriter") {
            self.has_message_writer = true;
        }
        syn::visit::visit_type_path(self, i);
    }

    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        // detect commands.spawn(), commands.entity(), asset_server.load(), audio.play(), .play()
        if let Expr::Path(ref p) = *node.receiver {
            if let Some(ident) = p.path.segments.last() {
                match ident.ident.to_string().as_str() {
                    "commands" | "asset_server" | "audio" => {
                        self.has_side_effect = true;
                    }
                    _ => {}
                }
            }
        } else {
            // receiver is some expression; still check method name
            let method = node.method.to_string();
            match method.as_str() {
                "spawn" | "entity" | "spawn_batch" | "insert_resource" | "despawn" | "play" => {
                    self.has_side_effect = true;
                }
                _ => {}
            }
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}

fn matches_message_writer(ty: &Type) -> bool {
    match ty {
        Type::Path(tp) => tp.path.segments.iter().any(|s| s.ident == "MessageWriter"),
        _ => false,
    }
}

pub fn analyze_file(path: &Path, src: &str) -> Vec<Finding> {
    let mut v = FnVisitor::new(src);
    let file: File = syn::parse_file(src).unwrap_or_else(|_| {
        // if parse fails, return empty
        File {
            shebang: None,
            attrs: Vec::new(),
            items: Vec::new(),
        }
    });
    v.visit_file(&file);
    // Note: item_fn check_and_reset uses in-memory path; we'll overwrite with real path
    let mut out = v.findings;
    for f in &mut out {
        f.file = path.to_string_lossy().to_string();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn flags_function_with_messagewriter_and_spawn() {
        let src = r#"
        fn example(mut writer: MessageWriter<Foo>, mut commands: Commands) {
            commands.spawn(());
        }
        "#;
        let tmp = NamedTempFile::new().unwrap();
        let p = tmp.path().to_path_buf();
        std::fs::write(&p, src).unwrap();
        let findings = analyze_file(&p, src);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].fn_name, "example");
    }

    #[test]
    fn does_not_flag_if_no_side_effect() {
        let src = r#"
        fn example(mut writer: MessageWriter<Foo>) {
            // just write
            writer.send(Foo{});
        }
        "#;
        let tmp = NamedTempFile::new().unwrap();
        let p = tmp.path().to_path_buf();
        std::fs::write(&p, src).unwrap();
        let findings = analyze_file(&p, src);
        assert_eq!(findings.len(), 0);
    }

    #[test]
    fn flags_on_play_call() {
        let src = r#"
        fn on_event(mut writer: MessageWriter<Foo>) {
            audio.play("boom");
        }
        "#;
        let tmp = NamedTempFile::new().unwrap();
        let p = tmp.path().to_path_buf();
        std::fs::write(&p, src).unwrap();
        let findings = analyze_file(&p, src);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].fn_name, "on_event");
    }
}
