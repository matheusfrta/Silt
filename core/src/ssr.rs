use crate::graph::G;
use std::collections::HashMap;

// fast native rust html generator
pub struct HtmlBuffer {
    buf: String,
}

impl HtmlBuffer {
    pub fn new() -> Self {
        Self { buf: String::with_capacity(4096) }
    }

    pub fn tag_open(&mut self, tag: &str, attrs: &HashMap<&str, &str>) {
        self.buf.push('<');
        self.buf.push_str(tag);
        for (k, v) in attrs {
            self.buf.push(' ');
            self.buf.push_str(k);
            self.buf.push_str("=\"");
            self.buf.push_str(v);
            self.buf.push('"');
        }
        self.buf.push('>');
    }

    pub fn tag_close(&mut self, tag: &str) {
        self.buf.push_str("</");
        self.buf.push_str(tag);
        self.buf.push('>');
    }

    pub fn text(&mut self, content: &str) {
        self.buf.push_str(content);
    }
    
    pub fn text_from_sig(&mut self, sig_id: usize) {
        if let Ok(v) = G.write().get(sig_id) {
            self.buf.push_str(&v.to_string());
        }
    }

    pub fn render(self) -> String {
        self.buf
    }
}