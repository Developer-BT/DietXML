DietXML
=======

Rapid development XML builder. Simple, fast, and maintainable hierarchical XML generation for .NET.

[![NuGet](https://img.shields.io/nuget/v/DietXML.svg)](https://www.nuget.org/packages/DietXML/)

License: MIT

DietXML simplifies building XMLs in C#
Makes deeply hierarchical XML generation simple, maintainable, and fast.
Define your XML structure in plain text, add elements one at a time, and have them automatically nested—no need for verbose DOM or LINQ-to-XML code.

In order to use find the library DietXML on Nuget.
Check option for pre-releases as still at version 0.2.0.
-------------------------------------------------------------------------------
Quick Reference
-------------------------------------------------------------------------------

## Version

**v0.2.0**  
⚠️ This is an early release. API will likely stay backward compatible at this point.

---

##  Planned Features

- Duplicate element names
- Performace tuning (good already but still room for improvement)
- Deeper formatting options


-------------------------------------------------------------------------------
Typical usage:
-------------------------------------------------------------------------------

Here we:
1. Declare a DietXML.XmlBuilder object.
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
// using directive required
using DietXML;

// create DietXML.XmlBuilder instance
XmlBuilder xb = new XmlBuilder();

// set the schema for the XML document
// this is done in plain text format
// tags must be balanced and well formed
// white space is ignored so indentation is not required but recommended
xb.SetSchema(@"
<root>
    <employee>
        <name></name>
    </employee>
</root>");


// now we can simply add elements to the XML document
// element names must match the schema
// (String element, String value)
xb.AddElement("name","John Doe");
xb.AddElement("name", "Jane Doe");


// at end you can build the xml
// this creates a sb item called output_xml on trhe object
xb.BuildXml();
Console.WriteLine(xb.output_xml);
        }
    }
}
```

Output:
```XML
<root>
  <employee>
    <name>John Doe</name>
    <name>Jane Doe</name>
  </employee>
</root>
```


Now we demonstrate a key strength of DietXML.
We can simply update the String definition of the schema and the parent attributes will implicitly be added.

```csharp

// using directive required
using DietXML;

// create DietXML.XmlBuilder instance
XmlBuilder xb = new XmlBuilder();

// updating the schema will automatically impact any elements without the need for extra code
xb.SetSchema(@"
<root>
    <employee>
       <parent3><parent2><parent1><name></name></parent1></parent2></parent3>
    </employee>
</root>");


xb.AddElement("name","John Doe");
xb.AddElement("name", "Jane Doe");


// at end you can build the xml
// this creates a sb item called output_xml on trhe object
xb.BuildXml();
Console.WriteLine(xb.output_xml);
```
Output
```XML
<root>
  <employee>
    <parent3>
      <parent2>
        <parent1>
          <name>John Doe</name>
          <name>Jane Doe</name>
        </parent1>
      </parent2>
    </parent3>
  </employee>
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
// using directive required
using DietXML;

// create DietXML.XmlBuilder instance
XmlBuilder xb = new XmlBuilder();

// set the schema for the XML document
// this is done in plain text format
// tags must be balanced and well formed
// white space is ignored so indentation is not required but recommended
xb.SetSchema(@"
<root>
    <employee>
       <parent1><name></name></parent1>
    </employee>
</root>");


// set key for employee
xb.SetKey("employee", "1");
xb.AddElement("name","John Doe");
// new key forces new employee element
xb.SetKey("employee", "2");
xb.AddElement("name", "Jane Doe");
// new key forces another new employee element
xb.SetKey("employee", "3");
xb.AddElement("name", "John Smith");


// at end you can build the xml
// this creates a sb item called output_xml on trhe object
xb.BuildXml();
Console.WriteLine(xb.output_xml);
```
Now we see the result of adding keys, with each new key forcing a new element of specified type.
Output:
```XML
<root>
  <employee>
    <parent1>
      <name>John Doe</name>
    </parent1>
  </employee>
  <employee>
    <parent1>
      <name>Jane Doe</name>
    </parent1>
  </employee>
  <employee>
    <parent1>
      <name>John Smith</name>
    </parent1>
  </employee>
</root>

```

You can also add attribrutes to element using the .Atrributes("propertyname1","propertyvalue1",""propertyname2","propertyvalue2")
This can be chained at the end of a SetKey() to attach to a parent attribute or AddElement() to attach at the deepest element.
Once an attribute is linked to a key, it is immutable (cannot be changed after).
Using the method again is fine, but only as long as the attribute values remain unchanged.

```csharp

using DietXML;

XmlBuilder xb = new XmlBuilder();

xb.SetSchema(@"
<root>
    <employee>
       <parent1><name></name></parent1>
    </employee>
</root>");



xb.SetKey("employee", "1").Attributes("id","1");  // attributes can be set at this point asscoiated by key
xb.AddElement("name","John Doe");

xb.SetKey("employee", "2").Attributes("id", "2"); ;
xb.AddElement("name", "Jane Doe");

xb.SetKey("employee", "3").Attributes("id", "1"); ;
xb.AddElement("name", "John Smith").Attributes("employee_number","1234", "user_name", "JS1"); // attributes can be set at this point asscoiated by deepest element

xb.BuildXml();
Console.WriteLine(xb.output_xml);
```

Output:
```XML
<root>
  <employee id="1">
    <parent1>
      <name>John Doe</name>
    </parent1>
  </employee>
  <employee id="2">
    <parent1>
      <name>Jane Doe</name>
    </parent1>
  </employee>
  <employee id="1">
    <parent1>
      <name employee_number="1234" user_name="JS1">John Smith</name>
    </parent1>
  </employee>
</root>
```


The last method used is the **clear_keys()** method.
This removes all active keys and sets them to the default group.

clear_keys():
```csharp
using DietXML;

XmlBuilder xb = new XmlBuilder();

xb.SetSchema(@"
<root>
    <employee>
       <parent1><name></name></parent1>
    </employee>
</root>");



xb.SetKey("employee", "1").Attributes("id","1");  
xb.ClearKeys();  // clear keys will remove any keys set for future elements, so the next element will not have any keys set
xb.AddElement("name","John Doe");

xb.SetKey("employee", "2").Attributes("id", "2"); ;
xb.ClearKeys();   // clear keys will remove any keys set for future elements, so the next element will not have any keys set
xb.AddElement("name", "Jane Doe");

xb.SetKey("employee", "3").Attributes("id", "3"); ;
//xb.ClearKeys();   // clear keys will remove any keys set for future elements, so the next element will not have any keys set
xb.AddElement("name", "John Smith").Attributes("employee_number","1234", "user_name", "JS1"); // attributes can be set at this point asscoiated by deepest element


// two element now added under one key
xb.SetKey("employee", "4").Attributes("id", "4");
xb.AddElement("name", "Mrs Example");
xb.AddElement("name", "Mr Example");


xb.BuildXml();
Console.WriteLine(xb.output_xml);
```

Output:
```XML
<root>
  <employee>
    <parent1>
      <name>John Doe</name>
      <name>Jane Doe</name>
    </parent1>
  </employee>
  <employee id="3">
    <parent1>
      <name employee_number="1234" user_name="JS1">John Smith</name>
    </parent1>
  </employee>
  <employee id="4">
    <parent1>
      <name>Mrs Example</name>
      <name>Mr Example</name>
    </parent1>
  </employee>
</root>
```





For contrast equivilent LINQ to XML code is less readable and more difficult to maintain

```csharp
     using System;
using System.Xml.Linq;

var root = new XElement("root");

// Employee 1
var employee1 = new XElement("employee",
    new XAttribute("id", "1"),
    new XElement("parent1",
        new XElement("name", "John Doe")
    )
);
root.Add(employee1);

// Employee 2
var employee2 = new XElement("employee",
    new XAttribute("id", "2"),
    new XElement("parent1",
        new XElement("name", "Jane Doe")
    )
);
root.Add(employee2);

// Employee 3
var employee3 = new XElement("employee",
    new XAttribute("id", "1"),
    new XElement("parent1",
        new XElement("name",
            new XAttribute("employee_number", "1234"),
            new XAttribute("user_name", "JS1"),
            "John Smith"
        )
    )
);
root.Add(employee3);

// Output the XML
Console.WriteLine(root);
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



Another comparison with Linq to visualise maintainability and development speed difference

DietXML:
```csharp
using DietXML;

// Example data arrays
string[][] employeeBasics = new[]
{
    new[] { "1", "Alice Smith", "Engineering" },
    new[] { "2", "Bob Jones", "Marketing" }
};

string[][] employeeContacts = new[]
{
    new[] { "1", "alice.smith@example.com", "555-1234" },
    new[] { "2", "bob.jones@example.com", "555-5678" }
};

string[][] employeeAddresses = new[]
{
    new[] { "1", "home", "USA", "123 Main St" },
    new[] { "1", "work", "USA", "456 Office Rd" },
    new[] { "2", "home", "Canada", "789 Maple Ave" }
};

string[][] employeeRoles = new[]
{
    new[] { "1", "Developer", "2015-01-01", "2018-12-31" },
    new[] { "1", "Lead Engineer", "2019-01-01", "" },
    new[] { "2", "Marketing Specialist", "2017-05-01", "2020-06-30" },
    new[] { "2", "Marketing Lead", "2020-07-01", "" }
};

string[][] transactions = new[]
{
    new[] { "1", "T1001", "2024-01-10", "Conference Registration" },
    new[] { "1", "T1002", "2024-02-05", "Team Lunch" },
    new[] { "2", "T2001", "2024-03-12", "Marketing Materials" }
};

string[][] lineItems = new[]
{
    new[] { "T1001", "Pencil", "10", "1.50" },
    new[] { "T1001", "Notebook", "2", "5.00" },
    new[] { "T1002", "Sandwich", "5", "7.00" },
    new[] { "T2001", "Brochure", "100", "0.75" }
};

// Define schema
var xb = new XmlBuilder();
xb.SetSchema(@"
<company>
  <employee>
    <profile>
      <name></name>
      <department></department>
    </profile>
    <contact>
      <email></email>
      <phone></phone>
    </contact>
    <addresses>
      <address>
        <line1></line1>
      </address>
    </addresses>
    <roles>
      <role>
        <title></title>
        <start_date></start_date>
        <end_date></end_date>
      </role>
    </roles>
    <transactions>
      <transaction>
        <date></date>
        <description></description>
        <lineitems>
          <item>
            <product></product>
            <quantity></quantity>
            <price></price>
          </item>
        </lineitems>
      </transaction>
    </transactions>
  </employee>
</company>
");

// Build employee data
for (int i = 0; i < employeeBasics.Length; i++)
{
    string id = employeeBasics[i][0];

    xb.SetKey("employee", id).Attributes("id", id);

    xb.AddElement("name", employeeBasics[i][1]);
    xb.AddElement("department", employeeBasics[i][2]);

    xb.AddElement("email", employeeContacts[i][1]);
    xb.AddElement("phone", employeeContacts[i][2]);
}

// Add addresses
foreach (var addr in employeeAddresses)
{
    xb.ClearKeys();
    xb.SetKey("employee", addr[0]);
    xb.SetKey("addresses", addr[1]);
    xb.SetKey("address", addr[1]).Attributes("type", addr[1], "country", addr[2]);
    xb.AddElement("line1", addr[3]);
}

// Add roles
foreach (var role in employeeRoles)
{
    xb.ClearKeys();
    xb.SetKey("employee", role[0]);
    xb.SetKey("roles", role[1]);
    xb.SetKey("role", role[1]);
    xb.AddElement("title", role[1]);
    xb.AddElement("start_date", role[2]);
    xb.AddElement("end_date", role[3]);
}

// Add transactions
foreach (var txn in transactions)
{
    xb.ClearKeys();
    xb.SetKey("employee", txn[0]);
    xb.SetKey("transactions", txn[1]);
    xb.SetKey("transaction", txn[1]).Attributes("transaction_id", txn[1]);
    xb.AddElement("date", txn[2]);
    xb.AddElement("description", txn[3]);
}

// Add line items
foreach (var item in lineItems)
{
    xb.ClearKeys();
    // Find employee for this transaction
    string txnId = item[0];
    string empId = null;
    foreach (var txn in transactions)
        if (txn[1] == txnId) empId = txn[0];

    if (empId == null) continue;

    xb.SetKey("employee", empId);
    xb.SetKey("transactions", txnId);
    xb.SetKey("transaction", txnId);
    xb.SetKey("lineitems", item[1]);
    xb.SetKey("item", item[1]);
    xb.AddElement("product", item[1]);
    xb.AddElement("quantity", item[2]);
    xb.AddElement("price", item[3]);
}

// Output
xb.BuildXml();
Console.WriteLine(xb.output_xml);
```

Linq to XML:
```csharp
using System;
using System.Linq;
using System.Xml.Linq;

// Example data arrays
string[][] employeeBasics = new[]
{
    new[] { "1", "Alice Smith", "Engineering" },
    new[] { "2", "Bob Jones", "Marketing" }
};

string[][] employeeContacts = new[]
{
    new[] { "1", "alice.smith@example.com", "555-1234" },
    new[] { "2", "bob.jones@example.com", "555-5678" }
};

string[][] employeeAddresses = new[]
{
    new[] { "1", "home", "USA", "123 Main St" },
    new[] { "1", "work", "USA", "456 Office Rd" },
    new[] { "2", "home", "Canada", "789 Maple Ave" }
};

string[][] employeeRoles = new[]
{
    new[] { "1", "Developer", "2015-01-01", "2018-12-31" },
    new[] { "1", "Lead Engineer", "2019-01-01", "" },
    new[] { "2", "Marketing Specialist", "2017-05-01", "2020-06-30" },
    new[] { "2", "Marketing Lead", "2020-07-01", "" }
};

string[][] transactions = new[]
{
    new[] { "1", "T1001", "2024-01-10", "Conference Registration" },
    new[] { "1", "T1002", "2024-02-05", "Team Lunch" },
    new[] { "2", "T2001", "2024-03-12", "Marketing Materials" }
};

string[][] lineItems = new[]
{
    new[] { "T1001", "Pencil", "10", "1.50" },
    new[] { "T1001", "Notebook", "2", "5.00" },
    new[] { "T1002", "Sandwich", "5", "7.00" },
    new[] { "T2001", "Brochure", "100", "0.75" }
};

var company = new XElement("company",
    employeeBasics.Select(emp =>
    {
        string empId = emp[0];

        // Addresses for this employee
        var addresses = employeeAddresses
            .Where(addr => addr[0] == empId)
            .Select(addr =>
                new XElement("address",
                    new XAttribute("type", addr[1]),
                    new XAttribute("country", addr[2]),
                    new XElement("line1", addr[3])
                )
            );

        // Roles for this employee
        var roles = employeeRoles
            .Where(role => role[0] == empId)
            .Select(role =>
                new XElement("role",
                    new XElement("title", role[1]),
                    new XElement("start_date", role[2]),
                    new XElement("end_date", role[3])
                )
            );

        // Transactions for this employee
        var empTransactions = transactions
            .Where(txn => txn[0] == empId)
            .Select(txn =>
            {
                string txnId = txn[1];
                // Line items for this transaction
                var items = lineItems
                    .Where(item => item[0] == txnId)
                    .Select(item =>
                        new XElement("item",
                            new XElement("product", item[1]),
                            new XElement("quantity", item[2]),
                            new XElement("price", item[3])
                        )
                    );
                return new XElement("transaction",
                    new XAttribute("transaction_id", txnId),
                    new XElement("date", txn[2]),
                    new XElement("description", txn[3]),
                    new XElement("lineitems", items)
                );
            });

        // Contact for this employee
        var contact = employeeContacts.FirstOrDefault(c => c[0] == empId);

        return new XElement("employee",
            new XAttribute("id", empId),
            new XElement("profile",
                new XElement("name", emp[1]),
                new XElement("department", emp[2])
            ),
            new XElement("contact",
                new XElement("email", contact?[1] ?? ""),
                new XElement("phone", contact?[2] ?? "")
            ),
            new XElement("addresses", addresses),
            new XElement("roles", roles),
            new XElement("transactions", empTransactions)
        );
    })
);

// Output
Console.WriteLine(company);
```

Output:
```XML
<company>
  <employee id="1">
    <profile>
      <name>Alice Smith</name>
      <department>Engineering</department>
    </profile>
    <contact>
      <email>alice.smith@example.com</email>
      <phone>555-1234</phone>
    </contact>
    <addresses>
      <address type="home" country="USA">
        <line1>123 Main St</line1>
      </address>
      <address type="work" country="USA">
        <line1>456 Office Rd</line1>
      </address>
    </addresses>
    <roles>
      <role>
        <title>Developer</title>
        <start_date>2015-01-01</start_date>
        <end_date>2018-12-31</end_date>
      </role>
      <role>
        <title>Lead Engineer</title>
        <start_date>2019-01-01</start_date>
        <end_date></end_date>
      </role>
    </roles>
    <transactions>
      <transaction transaction_id="T1001">
        <date>2024-01-10</date>
        <description>Conference Registration</description>
        <lineitems>
          <item>
            <product>Pencil</product>
            <quantity>10</quantity>
            <price>1.50</price>
          </item>
          <item>
            <product>Notebook</product>
            <quantity>2</quantity>
            <price>5.00</price>
          </item>
        </lineitems>
      </transaction>
      <transaction transaction_id="T1002">
        <date>2024-02-05</date>
        <description>Team Lunch</description>
        <lineitems>
          <item>
            <product>Sandwich</product>
            <quantity>5</quantity>
            <price>7.00</price>
          </item>
        </lineitems>
      </transaction>
    </transactions>
  </employee>
  <employee id="2">
    <profile>
      <name>Bob Jones</name>
      <department>Marketing</department>
    </profile>
    <contact>
      <email>bob.jones@example.com</email>
      <phone>555-5678</phone>
    </contact>
    <addresses>
      <address type="home" country="Canada">
        <line1>789 Maple Ave</line1>
      </address>
    </addresses>
    <roles>
      <role>
        <title>Marketing Specialist</title>
        <start_date>2017-05-01</start_date>
        <end_date>2020-06-30</end_date>
      </role>
      <role>
        <title>Marketing Lead</title>
        <start_date>2020-07-01</start_date>
        <end_date></end_date>
      </role>
    </roles>
    <transactions>
      <transaction transaction_id="T2001">
        <date>2024-03-12</date>
        <description>Marketing Materials</description>
        <lineitems>
          <item>
            <product>Brochure</product>
            <quantity>100</quantity>
            <price>0.75</price>
          </item>
        </lineitems>
      </transaction>
    </transactions>
  </employee>
</company>
```


<pre>
Why DietXML is Easier to Write
•	Schema-Driven Approach:
With DietXML, you define the entire XML structure up front using a schema. This means you don’t have to repeatedly specify the hierarchy or worry about element/attribute order in your code. You just set keys and add data.
•	Declarative Data Insertion:
You add data by simply setting the current context (with SetKey) and inserting values or attributes. There’s no need to manually construct or nest elements, which reduces boilerplate and cognitive load.
•	No Manual Tree Navigation:
You don’t have to traverse or manage parent/child relationships. DietXML handles all navigation for you based on the keys you set, so you can focus on the data, not the structure.
•	Attributes and Values Unified:
Adding attributes or values is always a single, clear call (Attributes or AddElement), and you can do this at any level without extra code.
Why DietXML is Exponentially Easier to Maintain
•	Centralized Structure:
The schema acts as a single source of truth for your XML layout. If you need to change the structure (add, remove, or move elements), you update the schema, not dozens of scattered code locations.
•	Minimal Code Changes for Structure Updates:
When requirements change (e.g., new nested elements, attributes, or deeper hierarchies), you rarely need to touch your data population logic. The schema absorbs most of the change.
•	Less Error-Prone:
There’s no risk of forgetting to add a parent element, misplacing a child, or duplicating code for similar structures. DietXML’s key-based navigation ensures data always lands in the right place.
•	Scales with Complexity:
As your XML grows in depth and breadth, DietXML code remains flat and readable. In contrast, LINQ to XML (or manual DOM code) becomes increasingly nested, verbose, and hard to follow.
•	Easier Onboarding:
New team members can quickly understand the XML structure by looking at the schema, rather than reverse-engineering nested code.

</pre>
