use serde::Serialize;
use swc_atoms::JsWord;
use swc_css_ast::{ImportHref, ImportPrelude, Url, UrlValue};
use swc_css_visit::{Visit, VisitWith};

pub struct Analyzer {
    pub deps: Dependencies,
}

#[derive(Debug, Serialize)]
pub struct Dependencies {
    pub imports: Vec<Import>,
    pub urls: Vec<CssUrl>,
}

#[derive(Debug, Serialize)]
pub struct Import {
    pub url: CssUrl,
}

#[derive(Debug, Serialize)]
pub struct CssUrl {
    pub value: JsWord,
}

impl Visit for Analyzer {
    fn visit_import_prelude(&mut self, n: &ImportPrelude) {
        n.layer_name.visit_with(self);
        n.import_conditions.visit_with(self);

        let url = normalize_import_href(&n.href);

        if let Some(url) = url {
            self.deps.imports.push(Import { url });
        }
    }

    fn visit_url(&mut self, n: &Url) {
        self.deps.urls.extend(normalize_url(n));
    }
}

fn normalize_import_href(n: &ImportHref) -> Option<CssUrl> {}

fn normalize_url(n: &Url) -> Option<CssUrl> {
    let v = &*n.value?;

    Some(CssUrl {
        value: match v {
            UrlValue::Str(v) => v.value.clone(),
            UrlValue::Raw(v) => v.value.clone(),
        },
    })
}
