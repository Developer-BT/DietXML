//! # diet-xml
//!
//! A schema-driven, ergonomic XML builder for Rust.
//!
//! ## Example
//! ```rust
//! use diet_xml::XmlBuilder;
//!
//! let mut xb = XmlBuilder::new();
//! xb.set_schema(r#"<root><item><value></value></item></root>"#);
//! xb.set_key("root", "1");
//! xb.set_key("item", "1");
//! xb.add_element("value", "42");
//! xb.build_xml();
//! println!("{}", xb.xml_out());
//! ```

mod builder;
mod schema;

// Public facing API
pub struct XmlBuilder {
    builder: builder::Builder,
}

impl XmlBuilder {
    pub fn new() -> Self {
        XmlBuilder {
            builder: builder::Builder::new(),
        }
    }

    pub fn set_schema(&mut self, txt_schema: &str) {
        self.builder.set_schema(txt_schema);
    }

    pub fn set_key(&mut self, nm_element: &str, txt_key: &str) -> ChainFromAdd {
        self.builder.set_key(nm_element, txt_key)
    }

    pub fn clear_keys(&mut self)  {
        self.builder.clear_key()
    }

    pub fn add_element(&mut self, nm_element: &str, value_element: &str) -> ChainFromAdd {
        self.builder.add_element(nm_element, value_element)
    }

    pub fn build_xml(&mut self) {
        self.builder.build_xml();
    }

    pub fn xml_out(&self) -> &str {
        &self.builder.xml_output
    }
}

// Re-export ChainFromAdd so users can call .attributes()
pub use builder::ChainFromAdd;