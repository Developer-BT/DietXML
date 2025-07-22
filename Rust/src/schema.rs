

#[derive(Debug,PartialEq)]
pub enum XmlSchemaError {
    InvalidTag(String),
    SchemaTxtInvalid(String),
    TagMismatch(String),
    DuplicateElementName(String),
    FirstTagIsClosing(String)
    // ... other error variants
}


    

pub(crate) struct XmlSchema
{
    raw_string: String,
   pub element_names: Vec<String>,
   pub element_position: Vec<Vec<usize>>,

    // parser state trackers
      buffer: String ,
       last_tag: Vec<String> ,
        ind_open: bool ,
        current_level: usize ,
        current_position: Vec<usize>,
     pub(crate)    element_no_lookup: fxhash::FxHashMap<String,Vec<usize>>,
}

    



// both validates the tag and returns whether it is an opening tag, false meaning a closing tag
    fn validate_tag_check_if_opening(txt_tag: &str) -> Result<bool,XmlSchemaError>
    {
  let mut ind_opening: bool = true;
  
  if txt_tag.is_empty() {
        return Err(XmlSchemaError::SchemaTxtInvalid("Schema Txt passed has length of zero".to_string()));
    }

  let mut no_skip: usize = 0 ;
  if txt_tag.starts_with('/')  { ind_opening = false; no_skip = 1;}




    for (i, c) in txt_tag.char_indices().skip(no_skip) {
        if !c.is_alphanumeric() && !(c == '-' && i > 0) && !(c == '_' && i > 0)&& !(c == '.' && i > 0)   {
            let msg = "Character: {} found in tag name: {} at position {}: not a valid tag".to_string();
            println!("{}",msg);
        return Err(XmlSchemaError::InvalidTag(msg));
            
        }
    }

    // Only return once, after the loop
    return Ok(ind_opening);
}
            
            
    
        // check either
        // "text" or  "/text" only alphanumerical and - allows

    
impl XmlSchema
{ 
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        Self {
            raw_string: String::new(),
            element_names: Vec::new(),
            element_position: Vec::new(),
              buffer:  String::new(),
             last_tag: Vec::new(),
             ind_open:  false,
           current_level: 0,
        current_position: Vec::new(),
        element_no_lookup: fxhash::FxHashMap::default(),
}
        }
    
    #[allow(dead_code)]    
    pub(crate) fn set_schema(&mut self, txt_schema: &str)
    {
        self.raw_string = txt_schema.to_string();
    }

    fn handle_close_tag(&mut self) -> Result<(), XmlSchemaError> 
    {
        // validates an checks whether an opening tag
    let ind_opening = validate_tag_check_if_opening(&self.buffer)?;

    if ind_opening
    {        
        //this is a new element that we need to add to list of element types
        match &mut self.element_names.contains(&self.buffer)
        {
            false => { self.element_names.push(self.buffer.clone());},
            true => { return Err(XmlSchemaError::DuplicateElementName(("Duplicate element".to_string()))) }
        }

        // now record the element level
        // new depth requires append
        let new_no_element = self.element_names.len() -1 ;
        if self.current_level >= self.current_position.len() { self.current_position.push(new_no_element);}    
        else {
            // update existing element
            self.current_position[self.current_level] = new_no_element;
            self.current_position.truncate(self.current_level + 1); // remove redundant positons
        }
        self.element_position.push(self.current_position.clone());
        self.element_no_lookup.insert(self.buffer.clone(), self.current_position.clone() );
        self.last_tag.push(self.buffer.clone());

       
        self.current_level += 1;

    }
    else {
          self.current_level -= 1;

     if let Some(last) = self.last_tag.pop() {
    // Check if the closing tag matches the last opened tag
    if last == &self.buffer[1..] {
        // Tags match, all good
    } else {
        return Err(XmlSchemaError::TagMismatch(format!(
            "Closing tag {} doesn't match last opening tag {}",
            &self.buffer, last
        )));
    }
} else {
    return Err(XmlSchemaError::FirstTagIsClosing(
        format!("Closing Tag {} found when no Tags open", &self.buffer),
    ));
}
        
    }

    return Ok(());
    }


    


#[allow(dead_code)]
pub(crate)fn parse_schema(&mut self)
{
    // Collect chars into a temporary vector to avoid borrowing self.raw_string during the loop
    let chars: Vec<char> = self.raw_string.chars().collect();

    for s in chars
    {
        if self.ind_open
        {
            match s {

            '<' => panic!("< symbol in open tag"),
            '>' => { /* save buffer with position;*/ 
                
                self.ind_open = false;
                self.handle_close_tag();
                self.buffer.clear();
               }
            // check for banned characters
            _ => { self.buffer.push(s); }
            } 
        }
        else
        {
            match s {
                c if c.is_whitespace() => continue,
                '<' => {self.ind_open = true},
                _ => panic!("Outside of tags, chracter: {} found",s)
            }
        }
    }
}
}
 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_schema() {
        let s = "<root> </root>";
        let mut schema = XmlSchema::new();
        schema.set_schema(s);
        // Test that it was set correctly
        assert_eq!(schema.raw_string, s);
        schema.parse_schema();
    }

    #[test]
        fn check_tag_validation() {



         assert_eq!(validate_tag_check_if_opening("validtag"), Ok(true));
         assert_eq!(validate_tag_check_if_opening("valid-tag"), Ok(true));
         assert_eq!(validate_tag_check_if_opening("/validclosing"), Ok(false));
         assert_eq!(validate_tag_check_if_opening("/valid-closing"), Ok(false));
         assert_eq!(validate_tag_check_if_opening("/valid-clo123sing"), Ok(false));

assert!(matches!(validate_tag_check_if_opening(""), Err(XmlSchemaError::SchemaTxtInvalid(_))));
assert!(matches!(validate_tag_check_if_opening("//invaliddoubleclose"), Err(XmlSchemaError::InvalidTag(_))));
//assert!(matches!(validate_tag_check_if_opening("invalidcharinmiddle"), Err(XmlSchemaError::InvalidTag(_))));
//assert!(matches!(validate_tag_check_if_opening(".atstart"), Err(XmlSchemaError::InvalidTag(_))));




 
        let l = "".to_string().len();
        println!("{}",l);
        
    }

    #[test]
    fn check_duplicate_detected()
    {
  let s = "<root> <field1> </field1> <1> <2> <3> <7> </7> <4> <5> </5> </4> </3> </2> </1> </root>";
        let mut schema = XmlSchema::new();
        schema.set_schema(s);
        // Test that it was set correctly
       schema.parse_schema();
       for r in &schema.element_names
        {println!("{}",r.to_string()) };
        

       for r in &schema.element_position
       {
        print!("{:?}",r)

       } 

    }

    
}
