

// use crate::schema::XmlSchema;
use crate::schema::XmlSchema;




#[derive(Debug)]
struct Element {
        position: Vec<usize>,
        value: String,       
         
}


pub(crate) struct Builder {
    elements: Vec<Element>,
    schema: XmlSchema,
    current_key: Vec<usize>,
    //lookup  String value of key, must be unique to (no_element, no_unique_key)
    key_list: fxhash::FxHashMap::<(usize,String),usize>,
    key_count: usize,
    pub xml_output: String,
    attribute_list: fxhash::FxHashMap::<(usize,usize),String>

    
}

pub struct ChainFromAdd<'a>
{
    builder: &'a mut Builder,
    no_element: usize,
    no_key: usize


}

impl<'a> ChainFromAdd<'a> {
    pub fn attributes(self, attributes: &[&str]) {
       
    // check attributes are pairs (even count)
    if &attributes.len()%2 != 0 
    {
        let msg = format!("Attributes are not value pair: {:?}", attributes) ;
        panic!("{}", msg );
    }

    // build all attributes from the pair values
    let mut all_attributes = "".to_string();
    for i in 0..attributes.len() /2
    {
        let att = attributes[i * 2];
        let value = format!("\"{}\"", attributes[i* 2 + 1]);
        let combined = format!(" {}={}", att, value);
        all_attributes.push_str(&combined);
    }

       match self.builder.attribute_list.get(&(self.no_element,self.no_key))
       {
        Some(value) => 
        {if &all_attributes != value 
            { panic!("Tried to add a second set of attributes to the same key/element")}
            // samme added again is just ignored and ok
        },
        None => { self.builder.attribute_list.insert((self.no_element, self.no_key), all_attributes.clone()) ; 
        
 //       println!("no_element: {}, no_key: {}, attributes: {}", self.no_element, self.no_key, all_attributes);
        }

       }
    }
}










impl Builder {

     pub(crate) fn new() -> Self
    {   
       
        Self {
         elements: vec![Element { position: Vec::new(), value: "".to_string() }],
         schema:  XmlSchema::new(),
         current_key: Vec::new(),
         key_list: fxhash::FxHashMap::default(),
         key_count: 0,
         xml_output: String::new(),
         attribute_list: fxhash::FxHashMap::default(),
        }
    }

    pub(crate) fn set_schema(&mut self,txt_schema: &str)
    {
        self.schema.set_schema(txt_schema);
        self.schema.parse_schema();
        self.current_key.resize(self.schema.element_no_lookup.len(), 0);
    }

    pub(crate) fn get_position(&self,nm_element: &str) -> &Vec<usize>
    {
     self.schema.element_no_lookup
    .get(nm_element)
    .expect("Tried to add an element that does not exist in the schema")
    }


        
 #[allow(unused_must_use)]   
pub(crate) fn set_key(&mut self, nm_element: &str, txt_key: &str) -> ChainFromAdd
{
    let position = self.get_position(nm_element);

    //let level = position.len() - 1;
    let &no_element = position.last().unwrap();
    let  returned_key: usize;

    // if same text key already exists for same element number then get the unique key ref and use again
    if let Some(&existing) = self.key_list.get(&(no_element,txt_key.to_string()))
    {
        self.current_key[no_element] = existing;
        returned_key = existing;
    }
        // if no matching key, then create new, key_count++ to ensure unqiue and ordered (based on order added)
    else {
        self.key_count += 1;
        self.key_list.insert((no_element, txt_key.to_string()),self.key_count);
        self.current_key[no_element] = self.key_count;
        returned_key = self.key_count;
    };


    return ChainFromAdd { builder: self, no_element: no_element, no_key: returned_key }

}

      

    pub(crate) fn clear_key(&mut self)
    {
        self.current_key.fill(0)  ;
    }

    #[allow(unused_must_use)]
    pub(crate) fn add_element(&mut self, nm_element: &str, value_element: &str) -> ChainFromAdd
    {
           let final_value = if value_element.is_empty() {
               " "
           } else {
               value_element
           };
           let position = self.get_position(nm_element).clone();
           self.key_count += 1;
           let new_key_count = self.key_count  ;
           self.current_key[*position.last().unwrap()] = new_key_count;
            //placeholder - add check to ensure is deepest element
           

           // new element always treated as a new key
           
           

           let positition_and_key = create_position(&position, &self.current_key);

           self.elements.push(Element{ position: positition_and_key, value: final_value.to_string()});

           return ChainFromAdd { builder: self, no_element: *position.last().unwrap(), no_key: new_key_count}
  
  
    }

    // combines position and key into a combined alternative Vec, this is what is sorted at teh end to create the final output

    pub(crate) fn build_xml(&mut self)
    {
        // add dummy row at end to enable last lags to be closed off
        // sort so element are all groups together , ordered by elements and keys
        self.elements.sort_by(|a, b| a.position.cmp(&b.position));
        self.elements.push(Element { position: Vec::new(), value: "".to_string() });

      
for n in 1..self.elements.len() {
    let last = &self.elements[n - 1];
    let current =  &self.elements[n];
   // println!("Last element is: {:?}",last);
   // println!("Current element is: {:?}",current);

    let len = last.position.len().max(current.position.len());

    // used to stored information for opening tags on pass through closing tags
    // (no_element, no_key, i (depth))
    let mut opening_tags: Vec<(usize,usize,usize)> = Vec::new();
    
    
    for i in (0..len/2).rev()  {
    
    
    let l = (last.position.get(2*i),last.position.get(2*i+1));     
    let c = (current.position.get(2*i),current.position.get(2*i+1));    

   // println!("Last pair is: {:?}",l);
  //  println!("Current pair is: {:?}",c);

    if l != c && l.0 != None {
   
        let elem_name = &self.schema.element_names[*l.0.unwrap()];
    
        let indent = if i == last.position.len()/2 - 1 {
    "".to_string()
} else {
    "  ".repeat(i)
};
          
   
       
         let close_tag = format!("{}</{}>\n",indent,elem_name);
         self.xml_output.push_str(&close_tag);
   // print!("{}", open_tag);
  
    } 
        //record opening tags on same pass through
        if l != c && c.0 != None {  opening_tags.push((*c.0.unwrap(),*c.1.unwrap(),i)) ;   }    
  
    } 

        // opening tags need to open from lowest level upwards so we need to iterate in reverse

       for &n in opening_tags.iter().skip(1).rev() {
  
      let elem_name = &self.schema.element_names[n.0];


     // fetch any attribute associated with the element,key combination else
     let attribute = self.attribute_list.get(&(n.0,n.1)) ;

     

    let open_tag = format!("{}<{}{}>\n", "  ".repeat(n.2),  elem_name,attribute.unwrap_or(&"".to_string()));
     self.xml_output.push_str(&open_tag);
   // print!("{}", open_tag);
}

// After printing all opening tags, print the value (if any)
// the check is necessary due to the last dummy element, maybe change this later on
if !current.value.is_empty() {
    let elem_name = &self.schema.element_names[opening_tags[0].0];
    let attribute = &self.attribute_list.get(&(opening_tags[0].0,opening_tags[0].1))  ;

  //   let indent = Builder::INDENT.repeat(n);
    let open_tag = format!("{}<{}{}>{}", "  ".repeat(opening_tags.first().unwrap().2), elem_name, attribute.unwrap_or(&"".to_string()), escape_xml(&self.elements[n].value));
   self.xml_output.push_str(&open_tag);
   // print!("{}", open_tag);
}


    }


    }
}


fn escape_xml(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            _ => escaped.push(c),
        }
    }
    escaped
}




 fn create_position(position: &Vec<usize>, key: &Vec<usize>) -> Vec<usize> {
    let len = position.len().max(key.len());
    let mut combined = Vec::with_capacity(len * 2);
    for i in position {
        
        let key_val = *key.get(*i).unwrap();
        // Combine however you want; here, for example, just sum:
        combined.push(*i);
        combined.push(key_val);
        // Or if you want to keep both, you could push one or the other, etc.
    }
    combined
}





#[cfg(test)]
mod tests {
    use super::*;
/*/
   //#[test]
   // changed functionality invalid test
    fn test_create_position()
    {
        //placehodler
        // update this test now an key list of all elements is used rather than
        // just level depth keys
        let position: Vec<usize> = [1,2,3,4,5,6].to_vec(); 
        let key: Vec<usize> = [0,1,2,3,4].to_vec();

        let combined  =  create_position(&position, &key);

        assert_eq!(combined,[1,0,2,1,3,2,4,3,5,4,6,0].to_vec() )

    }
*/
    #[test]
    fn test_set_key()
    {
        let mut xb: Builder = Builder::new();
        xb.set_schema(
            "<root>
                        <g1></g1>
                        <g2><g3><g4></g4></g3></g2></root>");
        
           for i in  &xb.schema.element_no_lookup
        {
            println!("{:?}",i);
        }
        xb.set_key("g2", "1");     
        println!("{:?}",xb.current_key);
        xb.set_key("g2", "1");
        println!("{:?}",xb.current_key);
        xb.set_key("g2", "2");
        println!("{:?}",xb.current_key);

        xb.set_key("g1", "1");     
        println!("{:?}",xb.current_key);
        xb.set_key("g1", "1");
        println!("{:?}",xb.current_key);
        xb.set_key("g1", "2");
        println!("{:?}",xb.current_key);

        xb.set_key("g2", "3");
        println!("{:?}",xb.current_key);
        xb.set_key("g2", "4");
        println!("{:?}",xb.current_key);

        xb.set_key("g4", "1");
        println!("{:?}",xb.current_key);

        xb.set_key("root", "0");
        println!("{:?}",xb.current_key);

        xb.add_element("g4", "999");
        println!("{:?}",xb.elements[0] );

        xb.clear_key();
        println!("{:?}",xb.current_key);


        xb.build_xml();
       



    }


}

