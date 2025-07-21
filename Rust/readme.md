# diet-xml

[![Crates.io](https://img.shields.io/crates/v/diet-xml.svg)](https://crates.io/crates/diet-xml)
[![Documentation](https://docs.rs/diet-xml/badge.svg)](https://docs.rs/diet-xml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)



A fast, schema-driven XML builder with an ergonomic API. Build XML documents from text defined schema with minimal boilerplate.

This is designed to be both high level and capable of handling complex structures in less code.
An emphasis on avoiding hard to maintain deeply nested code.
This library allows for rapid devlopement, easy to maintain procedural code and complex document generation.

### Why this library exists:
[diet-xml vs quick-xml comparison - see detailed example below](#comparison-section)


**diet-xml makes XML generation ridiculously simple.** Just define your XML structure once as a template, then fill in the data. 
-No complex syntax
-only configoration in human readable schema definition
-no fighting with nested builders. 

See the examples below - and be generating XML in under 5 minutes.

## Table of Contents
- [Quick Start](#quick-start-section)
- [Basic Usage](#basic-usage-section)
- [API Overview](#api-overview-section)
- [Examples](#examples-section)
  - [Basic Example](#basic-example-section)
  - [Parent Elements](#parent-elements-section)
  - [Multiple Elements](#multiple-elements-section)
  - [Using Keys](#using-keys-section)
  - [Adding Attributes](#attributes-section)
  - [Clear Keys](#clear-keys-section)
  - [Large Dataset](#large-dataset-section)
- [Why This Library Exists](#comparison-section)



> ⚠️ **Experimental**: This crate is in early development. API and features may change.
Planned featured
- Document headers
- Additional formatting options
- Error handling rework
- Heavy optimisation

<a name="quick-start-section"></a>
## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
diet-xml = "0.1.0-experimental"
```

<a name="basic-usage-section"></a>
## Basic Usage

See below in this read me for more detailed itroduction to library and runable examples

```rust
use diet_xml::XmlBuilder;
let mut xb = XmlBuilder::new();
xb.set_schema("<root><item><value></value></item></root>");
xb.set_key("root", "1");
xb.set_key("item", "1");
xb.add_element("value", "42");
xb.build_xml();
println!("{}", xb.xml_out());
```

**Output:**
```xml
<root>
  <item>
    <value>42</value>
  </item>
</root>
```

<a name="api-overview-section"></a>
## API Overview

- `XmlBuilder::new()` - Create a new XML builder
- `set_schema(schema)` - Define the XML structure template
- `set_key(element, key)` - Select which instance of an element to work with
- `add_element(name, value)` - Add content to an element
- `build_xml()` - Generate the final XML
- `xml_out()` - Get the resulting XML string

## Why diet-xml?

Traditional XML builders in Rust can be verbose and error-prone. diet-xml takes a different approach:

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing
**Note**: This crate is experimental. Use in production at your own risk.

<a name="examples-section"></a>
## Examples

<a name="basic-example-section"></a>
### Basic Example

```rust
use diet_xml::XmlBuilder;

fn main() {
    let mut xb: XmlBuilder = XmlBuilder::new();

    // schemas are defined in plan text but must well formed with matching Tag, this is validated by the library
    let txt_schema = 
    "<root>
        <department>
            <employee>
                <name></name>
            </employee>
        </department>
    </root>";

    xb.set_schema(txt_schema); // set schema
    xb.add_element("name", "John Dough"); // adds an element, (nm_element,element_value) both as &str, "name" must be contained in the schema, and must be a bottom level element
    xb.build_xml();  // builds a Str // accesses via xb.xml_out

    println!("{}", xb.xml_out());
}
```

**Output:**
```xml
<root>
  <department>
    <employee>
      <name>John Dough</name>
    </employee>
  </department>
</root>
```

Parent elements are implicitly built in diet-xml - we simply add them to the schema and they are automatically included.

<a name="parent-elements-section"></a>
### Parent Elements

Parent elements are automatically included in the document - you simply add them to the schema definition:

```rust
use diet_xml::XmlBuilder;

fn main() {
    let mut xb: XmlBuilder = XmlBuilder::new();

    // schemas are defined in plain text but must be well formed with matching tags, this is validated by the library
    let txt_schema = 
    "<root>
        <g1><g2><g3><g4><g5><g6>
            <department>
                <employee>
                    <name></name>
                </employee>
            </department>
        </g6></g5></g4></g3></g2></g1>
    </root>";

    xb.set_schema(txt_schema); // set schema
    xb.add_element("name", "John Dough"); 
    xb.build_xml();

    println!("{}", xb.xml_out());
}
```

**Output:**
``` xml
<root>
  <g1>
    <g2>
      <g3>
        <g4>
          <g5>
            <g6>
              <department>
                <employee>
                  <name>John Dough</name>
                </employee>
              </department>
            </g6>
          </g5>
        </g4>
      </g3>
    </g2>
  </g1>
</root>
```



<a name="multiple-elements-section"></a>
### Multiple Elements

This time we add multiple elements to the document:

```rust
use diet_xml::XmlBuilder;

fn main() {
    let mut xb: XmlBuilder = XmlBuilder::new();

    // schemas are defined in plain text but must be well formed with matching tags, this is validated by the library
    let txt_schema = 
    "<root>
        <department>
            <employee>
                <name></name>
            </employee>
        </department>
    </root>";

    xb.set_schema(txt_schema); // set schema
    xb.add_element("name", "John Dough"); 
    xb.add_element("name", "Jane Dough"); // 2nd element added
    xb.build_xml();  

    println!("{}", xb.xml_out());
}
```

**Output:** 
**Output:**

We see there are two elements added, but we see they are nested next to each other in the hierarchy.

```xml
``` xml
<root>
  <department>
    <employee>
      <name>John Dough</name>
      <name>Jane Dough</name>
    </employee>
  </department>
</root>
```

-This is because the parents all have the same keys, as such there is only one of each parent element

-Say we want to place these different <name> elements in seperate <employee> elements we must use the set key method
- This again takes text as arguments and you provide the nm_element (must be in schema) and key
- When you add an element after this, it will be grouped according to this key
= (whenever no group has been assigned, or has been clear and not assigned, everything will be group together in the same default group and appear first in the document order)

<a name="using-keys-section"></a>
### Using Keys

```rust
use diet_xml::XmlBuilder;

fn main() {
    let mut xb: XmlBuilder = XmlBuilder::new();

    // schemas are defined in plain text but must be well formed with matching tags, this is validated by the library
    let txt_schema = 
    "<root>
        <department>
            <employee>
                <name></name>
            </employee>
        </department>
    </root>";

    xb.set_schema(txt_schema); // set schema
    xb.set_key("employee", "1"); //set key
    xb.add_element("name", "John Dough"); 
    xb.set_key("employee", "0"); //set to another key - forces creation of a new employee element
    xb.add_element("name", "Jane Dough"); // 2nd element added
    xb.build_xml();  

    println!("{}", xb.xml_out());
}
```

**Output:**

```xml
<root>
  <department>
    <employee>
      <name>John Dough</name>
    </employee>
    <employee>
      <name>Jane Dough</name>
    </employee>
  </department>
</root>

```

<a name="attributes-section"></a>
### Attributes

This can either be chained to a `set_key` to apply to a parent element before adding the next element, or it can be added to the element itself, to be displayed on the deepest element (the one you pass a value on):

```rust
use diet_xml::XmlBuilder;

fn main() {

let mut xb: XmlBuilder = XmlBuilder::new();

// schemas are defined in plan text but must well formed with matching Tag, this is validated byvthe library
let txt_schema = 
"<root> <g1><g2><g3><g4><g5><g6>
    <department>
        <employee>
            <name> <name>
        </employee>
    </department>
    </g6></g5></g4></g3></g2></g1>
</root>

";

xb.set_schema(txt_schema); // set schema

//xb.set_key("employee", "1") ; 
xb.set_key("employee", "1").attributes(&["id","1","initials","JD" ]);
xb.add_element("name", "John Dough"); 
xb.set_key("employee", "2").attributes(&["id","2","initials","JD" ]);
xb.add_element("name", "Jane Dough").attributes(&["CITY","PARIS"]); 


xb.build_xml();

println!("{}", xb.xml_out());

}
```

``` xml
<root>
  <g1>
    <g2>
      <g3>
        <g4>
          <g5>
            <g6>
              <department>
                <employee id="1" initial="JD">
                  <name>John Dough</name>
                </employee>
                <employee id="2" initial="JD">
                  <name CITY="PARIS">Jane Dough</name>
                </employee>
              </department>
            </g6>
          </g5>
        </g4>
      </g3>
    </g2>
  </g1>
</root>
```

<a name="clear-keys-section"></a>
### Clear Keys

Here we clear keys after setting them. This demonstrates how clear keys resets the key grouping to a default. When building complicated structures, it can be best to clear keys at the end of iterations, before moving onto the next:

```rust
use diet_xml::XmlBuilder;

fn main() {
    let mut xb: XmlBuilder = XmlBuilder::new();

    // schemas are defined in plain text but must be well formed with matching tags, this is validated by the library
    let txt_schema = 
    "<root> 
        <department>
            <employee>
                <name></name>
            </employee>
        </department>
    </root>";
    
    xb.set_schema(txt_schema); // set schema
    xb.set_key("employee", "1");
    xb.clear_keys(); //clear key // previous set key ignored
    xb.add_element("name", "John Dough"); 
    xb.set_key("employee", "2");
    xb.clear_keys();  //clear key // previous set key ignored
    xb.add_element("name", "Jane Dough");

    xb.build_xml();
    println!("{}", xb.xml_out());
}
```

**Output:**

Here we see the keys were all ignored, due to the `clear_keys()` method being called:
``` xml
<root>
  <department>
    <employee>
      <name>John Dough</name>
      <name>Jane Dough</name>
    </employee>
  </department>
</root>
```

<a name="large-dataset-section"></a>
### Large Dataset
- here we will use a csv stored on the DietXML github page to produce a larger xml

<a name="comparison-section"></a>
## Comparison

### Why this library exists
- The below code is roughly equivilent in function to the quick-xml/serde example below
- diet-xml can do in a few dozen lines of code what takes hundreds of lines in some mainstream libraries  

## diet-xml verison


 ``` rust
use csv::Reader;
use std::io::Cursor;
use diet_xml::XmlBuilder;


fn main() {
    let url = "https://raw.githubusercontent.com/Developer-BT/DietXML/main/Rust/test_data/flights.csv";
    
    // Download CSV
    let csv_text = download_csv(url);
    
    // Parse CSV
    let csv_data = parse_csv(csv_text);
    
    // Show info
    println!("Headers: {:?}", csv_data.headers);
    println!("Got {} records", csv_data.records.len());

    let mut xb = XmlBuilder::new();
    let txt_schema = 
"<root>
    <airlines>
        <airline>
            <year>
                <flight>
                    <airports>
                        <origin></origin>
                        <destination></destination>
                    </airports>
                    <details>
                        <time></time>
                        <air_time></air_time>
                    </details>
                </flight>
            </year>
        </airline>
    </airlines>
</root>";

    xb.set_schema(txt_schema);

    for r in csv_data.iter() { 
        // get field values from csv record
        let name = r.get_field("name").unwrap();
        let carrier = r.get_field("carrier").unwrap();
        let year = r.get_field("year").unwrap();
        let id = r.get_field("id").unwrap();
        let flight = r.get_field("flight").unwrap();
        let origin = r.get_field("origin").unwrap();
        let destination = r.get_field("dest").unwrap();
        let time= r.get_field("time_hour").unwrap();
        let air_time= r.get_field("air_time").unwrap();
        
        

        // add to xml

        // set keys for each row
        xb.set_key("airline", name).attributes(&["carrier",carrier,"name",name]) ;
        xb.set_key("year", year).attributes(&["year",year]) ;
        xb.set_key("flight", id).attributes(&["id",id,"flight",flight]) ;
        // add elements with values
        xb.add_element("origin", origin);
        xb.add_element("destination", destination);
        xb.add_element("time", time);
        xb.add_element("air_time", air_time);

        // ready for next iteration
        xb.clear_keys();
        




    }

    xb.build_xml();
    println!("{}", xb.xml_out());
}




struct CsvRecord<'a> {
    record: &'a csv::StringRecord,
    headers: &'a csv::StringRecord,
}

impl<'a> CsvRecord<'a> {
    fn get_field(&self, column_name: &str) -> Option<&str> {
        for (i, header) in self.headers.iter().enumerate() {
            if header == column_name {
                return self.record.get(i);
            }
        }
        None
    }
}

struct CsvData {
    headers: csv::StringRecord,
    records: Vec<csv::StringRecord>,
}

impl CsvData {
    fn iter(&self) -> impl Iterator<Item = CsvRecord> {
        self.records.iter().map(move |record| CsvRecord {
            record,
            headers: &self.headers,
        })
    }
}

fn download_csv(url: &str) -> String {
    reqwest::blocking::get(url).unwrap().text().unwrap()
}

fn parse_csv(csv_text: String) -> CsvData {
    let mut reader = Reader::from_reader(Cursor::new(csv_text));
    let headers = reader.headers().unwrap().clone();
    let records: Vec<csv::StringRecord> = reader.records().map(|r| r.unwrap()).collect();
    
    CsvData { headers, records }
}
```

### Output

``` xml
...
    </airline>
    <airline carrier="F9" name="Frontier Airlines Inc.">
        <flight id="145" flight="835">
          <airports>
            <origin>LGA</origin>
            <destination>DEN</destination>
          </airports>
          <details>
            <time>2013-01-01 08:00:00</time>
            <air_time>257.0</air_time>
          </details>
        </flight>
        <flight id="592" flight="511">
          <airports>
            <origin>LGA</origin>
            <destination>DEN</destination>
          </airports>
          <details>
            <time>2013-01-01 17:00:00</time>
            <air_time>242.0</air_time>
          </details>
        </flight>
        <flight id="1023" flight="835">
          <airports>
            <origin>LGA</origin>
            <destination>DEN</destination>
          </airports>
          <details>
            <time>2013-01-02 08:00:00</time>
            <air_time>239.0</air_time>
          </details>
        </flight>
        <flight id="1516" flight="511">
          <airports>
            <origin>LGA</origin>
            <destination>DEN</destination>
          </airports>
          <details>
            <time>2013-01-02 17:00:00</time>
            <air_time>238.0</air_time>
          </details>
        </flight>
        <flight id="1963" flight="835">
          <airports>
            <origin>LGA</origin>
            <destination>DEN</destination>
          </airports>
          <details>
            <time>2013-01-03 08:00:00</time>
            <air_time>219.0</air_time>
          </details>
        </flight>
    </airline>
    <airline carrier="HA" name="Hawaiian Airlines Inc.">
        <flight id="162" flight="51">
          <airports>
            <origin>JFK</origin>
            <destination>HNL</destination>
          </airports>
          <details>
            <time>2013-01-01 09:00:00</time>
            <air_time>659.0</air_time>
          </details>
        </flight>
        <flight id="1073" flight="51">
          <airports>
            <origin>JFK</origin>
            <destination>HNL</destination>
          </airports>
          <details>
            <time>2013-01-02 09:00:00</time>
            <air_time>638.0</air_time>
          </details>
        </flight>
        <flight id="2018" flight="51">
          <airports>
            <origin>JFK</origin>
            <destination>HNL</destination>
          </airports>
          <details>
            <time>2013-01-03 09:00:00</time>
            <air_time>616.0</air_time>
          </details>
        </flight>
      </year>
    </airline>
  </airlines>
</root>
```



## quick-xml / serde verison
Cargo.toml

```
[dependencies]
serde = { version = "1.0", features = ["derive"] }
csv = "1.3.1"
diet-xml = { path = "C:\\Users\\PC\\Desktop\\dietSQL\\readschema" }
quick-xml = { version = "0.38", features = ["serialize"] }
reqwest = { version = "0.12.22", features = ["blocking"] }

```

``` rust
use csv::Reader;
use std::io::Cursor;
use serde::Serialize;
use quick_xml::se::to_string;
use std::collections::BTreeMap;

#[derive(Serialize)]
struct Root {
    airlines: Airlines,
}

#[derive(Serialize)]
struct Airlines {
    #[serde(rename = "airline")]
    airlines: Vec<Airline>,
}

#[derive(Serialize)]
struct Airline {
    #[serde(rename = "@carrier")]
    carrier: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "year")]
    years: Vec<Year>,
}

#[derive(Serialize)]
struct Year {
    #[serde(rename = "@year")]
    year: String,
    #[serde(rename = "flight")]
    flights: Vec<Flight>,
}

#[derive(Serialize)]
struct Flight {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@flight")]
    flight: String,
    airports: Airports,
    details: Details,
}

#[derive(Serialize)]
struct Airports {
    origin: String,
    destination: String,
}

#[derive(Serialize)]
struct Details {
    time: String,
    air_time: String,
}

fn build_structured_data(csv_data: CsvData) -> Root {
    // Group data by airline -> year -> flights (with sorting)
    let mut airline_groups: BTreeMap<String, BTreeMap<String, Vec<FlightData>>> = BTreeMap::new();
    
    // First pass: group all data
    for r in csv_data.iter() {
        let flight_data = FlightData {
            carrier: r.get_field("carrier").unwrap_or("").to_string(),
            name: r.get_field("name").unwrap_or("Unknown").to_string(),
            year: r.get_field("year").unwrap_or("0").to_string(),
            id: r.get_field("id").unwrap_or("").to_string(),
            flight: r.get_field("flight").unwrap_or("").to_string(),
            origin: r.get_field("origin").unwrap_or("").to_string(),
            destination: r.get_field("dest").unwrap_or("").to_string(),
            time: r.get_field("time_hour").unwrap_or("").to_string(),
            air_time: r.get_field("air_time").unwrap_or("").to_string(),
        };
        
        airline_groups
            .entry(flight_data.name.clone())
            .or_insert_with(BTreeMap::new)
            .entry(flight_data.year.clone())
            .or_insert_with(Vec::new)
            .push(flight_data);
    }

    // Sort flights within each year by flight number
    for (_, years) in airline_groups.iter_mut() {
        for (_, flights) in years.iter_mut() {
            flights.sort_by(|a, b| a.flight.cmp(&b.flight));
        }
    }

    // Convert grouped data to serde structs
    let airlines: Vec<Airline> = airline_groups
        .into_iter()
        .map(|(airline_name, years)| {
            let first_year = years.values().next().unwrap();
            let first_flight = first_year.first().unwrap();
            let carrier = first_flight.carrier.clone();

            let years: Vec<Year> = years
                .into_iter()
                .map(|(year_value, flights)| {
                    let flights: Vec<Flight> = flights
                        .into_iter()
                        .map(|flight_data| Flight {
                            id: flight_data.id,
                            flight: flight_data.flight,
                            airports: Airports {
                                origin: flight_data.origin,
                                destination: flight_data.destination,
                            },
                            details: Details {
                                time: flight_data.time,
                                air_time: flight_data.air_time,
                            },
                        })
                        .collect();

                    Year {
                        year: year_value,
                        flights,
                    }
                })
                .collect();

            Airline {
                carrier,
                name: airline_name,
                years,
            }
        })
        .collect();

    Root {
        airlines: Airlines { airlines },
    }
}

#[derive(Clone)]
struct FlightData {
    carrier: String,
    name: String,
    year: String,
    id: String,
    flight: String,
    origin: String,
    destination: String,
    time: String,
    air_time: String,
}

fn main() {
    let url = "https://raw.githubusercontent.com/Developer-BT/DietXML/main/Rust/test_data/flights.csv";
    
    let csv_text = download_csv(url);
    let csv_data = parse_csv(csv_text);
    
    println!("Headers: {:?}", csv_data.headers);
    println!("Got {} records", csv_data.records.len());

    // Build structured data with proper grouping and sorting
    let root = build_structured_data(csv_data);
    
    // Serialize to XML
    let xml = to_string(&root).unwrap();
    println!("{}", xml);
}

// ... rest of CSV handling code stays the same ...

struct CsvRecord<'a> {
    record: &'a csv::StringRecord,
    headers: &'a csv::StringRecord,
}

impl<'a> CsvRecord<'a> {
    fn get_field(&self, column_name: &str) -> Option<&str> {
        for (i, header) in self.headers.iter().enumerate() {
            if header == column_name {
                return self.record.get(i);
            }
        }
        None
    }
}

struct CsvData {
    headers: csv::StringRecord,
    records: Vec<csv::StringRecord>,
}

impl CsvData {
    fn iter(&self) -> impl Iterator<Item = CsvRecord> {
        self.records.iter().map(move |record| CsvRecord {
            record,
            headers: &self.headers,
        })
    }
}

fn download_csv(url: &str) -> String {
    reqwest::blocking::get(url).unwrap().text().unwrap()
}

fn parse_csv(csv_text: String) -> CsvData {
    let mut reader = Reader::from_reader(Cursor::new(csv_text));
    let headers = reader.headers().unwrap().clone();
    let records: Vec<csv::StringRecord> = reader.records().map(|r| r.unwrap()).collect();
    
    CsvData { headers, records }
}
```
