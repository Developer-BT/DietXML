use diet_xml::XmlBuilder;

fn main() {
    let mut xb = XmlBuilder::new();
    xb.set_schema(r#"<root><item><value></value></item></root>"#);
    xb.set_key("root", "1");
    xb.set_key("item", "1");
    xb.add_element("value", "42");
    xb.build_xml();
    println!("{}", xb.xml_out());
}
