DietXML
=======

Rapid development XML builder. Simple, fast, and maintainable hierarchical XML generation for .NET.

NuGet: https://www.nuget.org/packages/DietXML
License: MIT

DietXML simplifies building XMLs in C#
Makes deeply hierarchical XML generation simple, maintainable, and fast.
Define your XML structure in plain text, add elements one at a time, and have them automatically nested—no need for verbose DOM or LINQ-to-XML code.

-------------------------------------------------------------------------------
Quick Reference
-------------------------------------------------------------------------------

| Method            | Description |
|------------------|-------------|
| `set_schema(string)` | Define the XML structure/schema as a plain text string. <br> **Example:** `xb.set_schema("<root><item></item></root>");` |
| `add_element(name, value)` | Add an element (must be defined in the schema) with a value. <br> **Example:** `xb.add_element("item", "value");` |
| `set_key(name, key)` | Set a key for a parent/grouping element to control nesting/repetition. <br> **Example:** `xb.set_key("level1", "myKey");` |
| `clear_keys()` | Clear all keys, so subsequent elements use default/shared parents. <br> **Example:** `xb.clear_keys();` |
| `build_xml()` | Build the XML document. Populates `output_xml` with the result. <br> **Example:** `xb.build_xml();` |
| `output_xml` | The `StringBuilder` containing the generated XML after `build_xml()`. <br> **Example:** `Console.WriteLine(xb.output_xml.ToString());` |


## Version

**v0.1.0**  
⚠️ This is an early release. The API is experimental and may change in the future.

---

##  Planned Features

- Adding properties to parent attributes  
- Custom sorting on structure  
- Methods to convert to standard XML  
- Stronger validation on schema  
- Tuning parameters to process huge structures faster


-------------------------------------------------------------------------------
Typical usage:
-------------------------------------------------------------------------------

Here we:
1. Declare a DietXML.Xml_builder object.
2. Define the schema in plain text
3. Add some elements
4. Produce the output 

```csharp
using System;
using DietXML;
//use namespace
namespace DietXML
{
    class Program
    {
        static void Main(string[] args)
        {
            // create XmlBuilder instance 
            XmlBuilder xb = new XmlBuilder();

            // set schema using plain text
            // must be well-formed and balanced
            xb.set_schema("<name></name>");

            // add elements to the XML document
            // pass the element name as a string - this must be the same as the element name in the schema
            // pass the element value as a string
            xb.add_element("name", "John Doe");
            xb.add_element("name", "Jane Smith");
            xb.add_element("name", "Jane Doe");
            xb.add_element("name", "John Smith");

            // build the XML document - this sets xb.output_xml to a StringBuilder containing the XML document
            xb.build_xml();

            // print for demo
            Console.WriteLine(xb.output_xml.ToString());
        }
    }
}
```
```
Output:
Sorting XML document
Building XML Tags
<name>John Doe</name>
<name>Jane Smith</name>
<name>Jane Doe</name>
<name>John Smith</name>
```


Now we demonstrate the a key strength of DietXML.
We can simply update the String definition of the schema and the parent attributes will implicitly be added.

``` csharp

using System;
using DietXML;
namespace DietXML
{
    class Program
    {
        static void Main(string[] args)
        {
            // create XmlBuilder instance 
            XmlBuilder xb = new XmlBuilder();

            // we wrap the <name> element in opening and closing brackets for multiple element types
            xb.set_schema(@"
            <root>
            <level1>
                <level2>
                    <level3>
                        <level4>
                            <level5>
                              <level6>
                                <name></name>
                                </level6>
                            </level5>
                        </level4>
                    </level3>
                </level2>
            </level1>
            </root>
");

            // we do not need to add the parent element, they will be implicitly built
            xb.add_element("name", "John Doe");
            xb.add_element("name", "Jane Smith");
            xb.add_element("name", "Jane Doe");
            xb.add_element("name", "John Smith");

            // build the XML document - this sets xb.output_xml to a StringBuilder containing the XML document
            xb.build_xml();

            // print for demo
            Console.WriteLine(xb.output_xml.ToString());
        }
    }
}
```
Output
```
Sorting XML document
Building XML Tags
<root>
  <level1>
    <level2>
      <level3>
        <level4>
          <level5>
            <level6>
              <name>John Doe</name>
              <name>Jane Smith</name>
              <name>Jane Doe</name>
              <name>John Smith</name>
            </level6>
          </level5>
        </level4>
      </level3>
    </level2>
  </level1>
</root>
```

We see in the output that the parent elements are now created.
However the <name> elements have just been added together to the <level6> element which brings us onto the concept of Keys

We can set keys on parent elements.
This must be done BEFORE adding the element.

Adding keys to parent elements will seperate/group parent elements based on different/matching keys.
With no key is added everything willshare a default key (ie be grouped)

Using keys:

```csharp
using System;
using DietXML;
//use namespace
namespace DietXML
{
    class Program
    {
        static void Main(string[] args)
        {
            // create XmlBuilder instance 
            XmlBuilder xb = new XmlBuilder();

            // we wrap the <name> element in opening and closing brackets for multiple element types
            xb.set_schema(@"
            <root>
            <level1>
                <level2>
                    <level3>
                        <level4>
                            <level5>
                              <level6>
                                <name></name>
                                </level6>
                            </level5>
                        </level4>
                    </level3>
                </level2>
            </level1>
           
            </root>
");
            // we do not need to add the partent element, they will be implicitly built
            xb.add_element("name", "John Doe");

            // sets the key ready for the next element
            // in this case the key is being set on the lebvel1 element
            // the key must be string
      //      xb.set_key("level1", "1");
            // now we add this element and it is tied to a parent <level1> based on this key        
            xb.add_element("name", "Jane Smith");


            // we set another key, as this is a different key on level1, it will create a new <level1> element in the XML document
            // but only if an element is added after this key is set
            xb.set_key("level1", "any_string_is_fine");
            // now we add this element and it is tied to a parent <level1> based on this key    

        //    xb.set_key("level1", "key_with_multiple_lements");
            xb.add_element("name", "1st name under same level1");
            xb.add_element("name", "2nd name under same level1");

            // build the XML document - this sets xb.output_xml to a StringBuilder containing the XML document
            xb.build_xml();

            // print for demo
            Console.WriteLine(xb.output_xml.ToString());
        }
    }
}
```
Now we see the result of adding keys
Output:
```
Sorting XML document
Building XML Tags
<root>
  <level1>
    <level2>
      <level3>
        <level4>
          <level5>
            <level6>
              <name>John Doe</name>
              <name>Jane Smith</name>
            </level6>
          </level5>
        </level4>
      </level3>
    </level2>
  </level1>
  <level1>
    <level2>
      <level3>
        <level4>
          <level5>
            <level6>
              <name>1st name under same level1</name>
              <name>2nd name under same level1</name>
            </level6>
          </level5>
        </level4>
      </level3>
    </level2>
  </level1>
</root>

```
For contrast the equivilent LINQ to XML code is less readable are more difficult to maintain
```csharp
      var root = new XElement("root");

      // Dictionary to manage <level1> elements by key
      var level1Dict = new Dictionary<string, XElement>();

      // Helper to create the full hierarchy under <level1>
      XElement CreateFullHierarchy(XElement level1Parent)
      {
          var level2 = new XElement("level2");
          var level3 = new XElement("level3");
          var level4 = new XElement("level4");
          var level5 = new XElement("level5");
          var level6 = new XElement("level6");
          level5.Add(level6);
          level4.Add(level5);
          level3.Add(level4);
          level2.Add(level3);
          level1Parent.Add(level2);
          return level6;
      }

      // 1. First <name> (no key, so use a default key)
      var defaultKey = "__default__";
      var level1_0 = new XElement("level1");
      var level6_0 = CreateFullHierarchy(level1_0);
      level6_0.Add(new XElement("name", "John Doe"));
      root.Add(level1_0);
      level1Dict[defaultKey] = level1_0;

      // 2. <name> under the same <level1> as the first (no new key set)
      // We'll just use the same defaultKey for demonstration
      var level6_1 = level1Dict[defaultKey].Descendants("level6").First();
      level6_1.Add(new XElement("name", "Jane Smith"));

      // 3. <name> under a new <level1> with key "any_string_is_fine"
      var key2 = "any_string_is_fine";
      var level1_2 = new XElement("level1");
      var level6_2 = CreateFullHierarchy(level1_2);
      level6_2.Add(new XElement("name", "1st name under same level1"));
      level6_2.Add(new XElement("name", "2nd name under same level1"));
      root.Add(level1_2);
      level1Dict[key2] = level1_2;

      // Output the XML
      var doc = new XDocument(root);
      Console.WriteLine(doc.ToString());
```

dietXML offers several advantages over LINQ to XML for XML generation in .NET, especially for scenarios involving deeply hierarchical or high-volume XML documents:
---
1. Simplicity and Speed of Development
•	Schema-Driven: Define your XML structure as a plain text schema. No need to manually build element trees or manage parent/child relationships in code.
•	Minimal API: Add elements by name and value; parent elements are created automatically based on the schema.
•	Less Boilerplate: No need to write repetitive code for each level of nesting.
2. Automatic Hierarchy Management
•	Implicit Parent Creation: Parent and ancestor elements are generated automatically when you add a child element, reducing manual tree management.
•	Key-Based Grouping: Easily create repeating groups (e.g., multiple <level1> elements) using keys, without complex code.
3. Performance and Memory Efficiency
•	Batch-Oriented: Designed for high-throughput scenarios, with efficient memory allocation and minimal object creation.
•	Optimized for Large Data: Handles large numbers of elements efficiently, avoiding the overhead of building and traversing full object trees.
4. Maintainability
•	Schema as Documentation: The schema string serves as both code and documentation for your XML structure.
•	Easy Refactoring: Changing the XML structure is as simple as updating the schema string.
5. Reduced Error Surface
•	No Manual Tree Manipulation: Avoids common mistakes with manual parent/child management, such as misplaced or missing elements.
•	Validation: Can validate element names and structure against the schema at runtime.
---
When to Prefer dietXML
•	When you need to generate deeply nested or repetitive XML quickly.
•	When your XML structure is known up front and can be described as a schema.
•	When you want to minimize code complexity and maximize maintainability.
•	When performance and memory usage are important (e.g., large exports, ETL, reporting).


The last method uses is the **clear_keys()** method.
This removes all active keys and sets them to the default group.

clear_keys():
```csharp
using System;
using DietXML;
//use namespace
namespace DietXML
{
    class Program
    {
        static void Main(string[] args)
        {
            // create XmlBuilder instance 
            XmlBuilder xb = new XmlBuilder();

            // we wrap the <name> element in opening and closing brackets for multiple element types
            xb.set_schema(@"
            <root>
            <level1>
                <level2>
                    <level3>
                        <level4>
                            <level5>
                              <level6>
                                <name></name>
                                </level6>
                            </level5>
                        </level4>
                    </level3>
                </level2>
            </level1>
            </root>
");

            xb.add_element("name", "John Doe");

            xb.set_key("level1", "1");                
            xb.add_element("name", "Jane Smith");

            xb.set_key("level1", "100"); // ignored due to clear_keys()
            xb.clear_keys();
            // this clears all keys set so far, anything in this state will share all parent keys
            // essentially a default key

            // added with no keys
            xb.add_element("name", "1st name after clearing key");

            xb.set_key("level1", "100"); // ignored due to clear_keys()
            xb.clear_keys();

            xb.add_element("name", "2nd name after clearing key");

            // build the XML document - this sets xb.output_xml to a StringBuilder containing the XML document
            xb.build_xml();

            // print for demo
            Console.WriteLine(xb.output_xml.ToString());

        }
    }
}
```

Output:
```
<root>
  <level1>
    <level2>
      <level3>
        <level4>
          <level5>
            <level6>
              <name>John Doe</name>
              <name>1st name after clearing key</name>
              <name>2nd name after clearing key</name>
            </level6>
          </level5>
        </level4>
      </level3>
    </level2>
  </level1>
  <level1>
    <level2>
      <level3>
        <level4>
          <level5>
            <level6>
              <name>Jane Smith</name>
            </level6>
          </level5>
        </level4>
      </level3>
    </level2>
  </level1>
</root>
```





Another comparison with Linq to visualise maintainability and development speed difference

DietXML:
```csharp
string schema = @"
<root>
  <customer_type>
    <customer>
      <names>
        <first></first>
        <last></last>
      </names>
      <address>
        <line1></line1>
        <line2></line2>
        <line3></line3>
        <line4></line4>
      </address>
      <orders>
        <order_number></order_number>
      </orders>
    </customer>
  </customer_type>
</root>
";

XmlBuilder xb = new XmlBuilder();
xb.set_schema(schema);

// Process people
foreach (var person in people)
{
    xb.set_key("customer_type", person["customer_type"]);
    xb.set_key("customer", person["tp_customer"]);
    xb.add_element("first", person["first"]);
    xb.add_element("last", person["last"]);
}

// Process addresses
foreach (var addr in addresses)
{
    xb.set_key("customer", addr["tp_customer"]);
    xb.add_element("line1", addr["line1"]);
    xb.add_element("line2", addr["line2"]);
    xb.add_element("line3", addr["line3"]);
    xb.add_element("line4", addr["line4"]);
}

// Process orders
foreach (var order in orders)
{
    xb.set_key("customer", order["tp_customer"]);
    xb.add_element("order_number", order["order_number"]);
}

xb.build_xml();
Console.WriteLine(xb.output_xml.ToString());
```

Linq to XML:
```csharp
using System.Linq;
using System.Xml.Linq;

// Group people by customer_type, then by tp_customer
var customersByType = people
    .GroupBy(p => p["customer_type"])
    .ToDictionary(
        g => g.Key,
        g => g.ToDictionary(
            p => p["tp_customer"],
            p => p
        )
    );

var addressesByCustomer = addresses
    .GroupBy(a => a["tp_customer"])
    .ToDictionary(g => g.Key, g => g.First());

var ordersByCustomer = orders
    .GroupBy(o => o["tp_customer"])
    .ToDictionary(g => g.Key, g => g.ToList());

var root = new XElement("root",
    customersByType.Select(ct =>
        new XElement("customer_type",
            ct.Value.Select(custKvp =>
            {
                var cust = custKvp.Value;
                var custId = custKvp.Key;

                var namesElem = new XElement("names",
                    new XElement("first", cust["first"]),
                    new XElement("last", cust["last"])
                );

                XElement addressElem = new XElement("address");
                if (addressesByCustomer.TryGetValue(custId, out var addr))
                {
                    addressElem.Add(
                        new XElement("line1", addr["line1"]),
                        new XElement("line2", addr["line2"]),
                        new XElement("line3", addr["line3"]),
                        new XElement("line4", addr["line4"])
                    );
                }

                XElement ordersElem = new XElement("orders");
                if (ordersByCustomer.TryGetValue(custId, out var custOrders))
                {
                    foreach (var order in custOrders)
                    {
                        ordersElem.Add(new XElement("order_number", order["order_number"]));
                    }
                }

                return new XElement("customer", namesElem, addressElem, ordersElem);
            })
        )
    )
);

Console.WriteLine(root);
``
