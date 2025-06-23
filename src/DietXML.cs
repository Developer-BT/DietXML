// xml_builder.cs
// A high-performance XML builder that constructs XML documents from flat data using a custom schema.
// Supports hierarchical key management, element sorting, and efficient memory usage.
// .NET 8, C# 12

using System.Collections.Generic;
using System.Text;
namespace DietXML
{
    public class XmlBuilder
    {
        private DietXmlSchema schema = new DietXmlSchema();
        // Stores all elements to be included in the XML document.
        private List<element> elements = new List<element>(size_of_batch);

        // Tracks the current key path for hierarchical elements.
        private List<int> current_key = new List<int>();

        // Batch size for resizing the elements list.
        const int size_of_batch = 100000;
        // currently set to 5000000, some additional tuning parameters need to be added so user can appropiately size
        // set to 100,000 for now to prevent repeatedly extending and havig to allocate more memory to dictionary
        int no_added_in_batches = size_of_batch;

        // Tracks the number of elements added.
        int no_of_elements_added = 0;

        // Output buffer for the generated XML.

        public StringBuilder output_xml = new StringBuilder();

        // Maps string keys to unique integer IDs for key management.
        internal Dictionary<string, int> key_list = new Dictionary<string, int>(5000000);
        // currently set to 5000000, some additional tuning parameters need to be added so user can appropiately size
        // set to 5 mill for now to prevent repeatedly extending and havig to allocate more memory to dictionary


        // Temporary storage for element positions and keys.
        List<int> position_with_keys = new List<int>();
        int[] position;

        // Represents a single XML element with its text content and position info.
        private struct element
        {

            // how individual elements are stored
            // txt_element is the text content of the element
            // positons represents a combination of the position in the schema and the key for that position
            // odd array positions are the element numbers, even are the keys for repeated elements and their parent elements
            // eg
            // <Month="Decemeber" key="1,1,12,12">    no_elem, no_key, no_elem, no_key....

            // here 1 might represesnt say the year and the second 1 that it is the first year key in this process
            // the 12 that is a month element, and the 2nd 12 that is is the 12th key used for Month.
            // These element/position arrays can then be used to sort the elements in the exact order they will in the xml document

            // The opening and closing element tags can be derived by look at the current element and back to the previous element/forward to the next and comparing what is new/absent

            internal readonly int[] position;
            internal readonly string txt_element;
            internal readonly string properties;

            internal element(String txt_element, int[] position)
            {
                this.txt_element = txt_element;
                this.position = position;
                properties = null;
            }

          

            internal int get_level()
            {
                return (position.Count() - 2) / 2;
            }
            internal int get_position(int level)
            {
                int index = level * 2;
                if (index < 0 || index >= position.Length)
                    throw new IndexOutOfRangeException($"Requested position index {index} is out of bounds for element.");
                return position[index];
            }
            internal int get_key(int level)
            {
                return position[level * 2 + 1];
            }
        }

        // set the text format schema
        public void set_schema(string txt_schema)
        {
            schema.set_schema(txt_schema);
        }


        // Adds an element to the XML document, recording its value and position.


        /// <summary>
        /// Adds an element to the XML document, recording its value and position.
        /// </summary>
        /// <param name="nm_element">The name of the element as defined in the schema.</param>
        /// <param name="txt_element">The text content of the element.</param>
        public void add_element(string nm_element, string txt_element)
        {
            if (schema.txt_schema == null)
            {
                throw new InvalidOperationException("Schema is not set. Call set_schema() before adding elements.");
            }


            txt_element = XmlEscape(txt_element);


            if (no_of_elements_added >= no_added_in_batches - 1)
            {
                elements.Capacity += size_of_batch;
                no_added_in_batches += size_of_batch;
            }

            position_with_keys.Clear();

            try
            {
                position = schema.read_position(nm_element);
            }
            catch (KeyNotFoundException)
            {
                throw new ArgumentException($"Element name '{nm_element}' not found in schema.");
            }

            int currentKeyCount = current_key.Count;
            int positionCount = position.Length;
            int maxCount = Math.Max(currentKeyCount, positionCount);

            for (int j = 0; j < maxCount; j++)
            {
                position_with_keys.Add(j < positionCount ? position[j] : 0);
                position_with_keys.Add(j < currentKeyCount ? current_key[j] : 0);
            }

            no_of_elements_added++;
            this.elements.Add(new element(txt_element, position_with_keys.ToArray()));
        }

        // Sets the key for a given element, managing hierarchical key paths.


        /// <summary>
        /// Sets the key value for a given element, managing the hierarchical key path.
        /// </summary>
        /// <param name="key">The name of the key as defined in the schema (e.g., a parent or grouping element).</param>
        /// <param name="value">The value to assign to this key for the current element context.</param>
        /// <exception cref="InvalidOperationException">
        /// Thrown if the schema has not been set. Call set_schema() before setting keys.
        /// </exception>
        public void set_key(string key, string value)
        {
            if (schema.txt_schema == null) { throw new InvalidOperationException("Schema is not set. Call set_schema() before setting keys."); }


            int level = schema.read_level(key);
            while (this.current_key.Count() < level + 1)
            {
                current_key.Add(0);
            }

            if (key_list.ContainsKey(value))
            {
                current_key[level] = key_list[value];
            }
            else
            {
                key_list.Add(value, key_list.Count + 1);
                current_key[level] = key_list[value];
            }
            current_key[level] = key_list[value];
        }


        /// <summary>
        /// Clears (resets) the key value for the specified element level in the current key path.
        /// </summary>
        /// <param name="key">The name of the key as defined in the schema to clear.</param>
        /// <exception cref="InvalidOperationException">
        /// Thrown if the schema has not been set. Call set_schema() before clearing keys.
        /// </exception>
        public void clear_keys()
        {
            if (schema.txt_schema == null) { throw new InvalidOperationException("Schema is not set. Call set_schema() before clearing keys."); }
            this.current_key.Clear();
        }

        // Gets the element name by its index in the schema.
        private string get_elem_name(int no_elem)
        {
            return schema.array_elem_names[no_elem];
        }

        // Generates the XML output by sorting and printing all elements.


        /// <summary>
        /// Generates the XML output by sorting and printing all added elements according to the schema and key hierarchy.
        /// </summary>
        /// <exception cref="InvalidOperationException">
        /// Thrown if no elements have been added to the XML document.
        /// </exception>
        public void build_xml()
        {
            if (elements.Count == 0)
            {
                throw new InvalidOperationException("No elements have been added to the XML document.");
            }
          //  schema.print_schema();
            Console.WriteLine("Sorting XML document");
            sort_elements_by_position();
            Console.WriteLine("Building XML Tags");
            int no_elems = schema.array_string_element_positions.Count;
            StringBuilder buffer = new StringBuilder();


            // used to record and then compare current and last element positions
            List<int> position = new List<int>();
            List<int> last_position = new List<int>();

            element current_element;
            element last_element;

          

            current_element = elements[0];
            int current_no_levels = current_element.get_level();
            int last_no_levels;
  

            // Handle the first element seperately to initialize the XML structure - else would error on  elements[i - 1] as i = 0
            foreach (int i in Enumerable.Range(0, current_no_levels))
            {
                string nm_element = schema.array_elem_names[current_element.get_position(i)];
                output_xml.AppendLine($"{new string(' ', i * 2)}<{nm_element}>");
            }

            int level = current_element.get_level();
            int elemIndex = current_element.get_position(level);
            string elemName = schema.array_elem_names[elemIndex];
            output_xml.AppendLine($"{new string(' ', level * 2)}<{elemName}>{current_element.txt_element}</{elemName}>");



            // now handle all elements that are not the first or last element
            for (int i = 1; i < elements.Count; i++)
            {
                last_element = elements[i - 1];
                current_element = elements[i];

                last_no_levels = last_element.get_level();
                current_no_levels = current_element.get_level();


                // first we handle closing the last element tags that are no longer needed

                // 1. find the first level where the path diverges (ie either an element or it's key is different)
                int diffLevel = 0;
                int minLevels = Math.Min(last_no_levels, current_no_levels);
                for (; diffLevel < minLevels; diffLevel++)
                {
                    if (last_element.get_position(diffLevel) != current_element.get_position(diffLevel) ||
                        last_element.get_key(diffLevel) != current_element.get_key(diffLevel))
                    {
                        break;
                    }
                }

                // 2. Close all tags from the last element's deepest level down to diffLevel
                for (int j = last_no_levels - 1; j >= diffLevel; j--)
                {
                    string nm_element = schema.array_elem_names[last_element.get_position(j)];
                    output_xml.AppendLine($"{new string(' ', j * 2)}</{nm_element}>");
                }


                // then we handle opening the current element tags that are needed

                // creating tags starts with the highest level and works its way up to the deepest nesting level
                // eg <root><level1><level>.
                // 1. Find the first level where the path diverges
                diffLevel = 0;
                minLevels = Math.Min(last_no_levels, current_no_levels); // to ensure cases where current/last element positions are of different array length
                for (; diffLevel < minLevels; diffLevel++)
                {
                    if (last_element.get_position(diffLevel) != current_element.get_position(diffLevel) ||
                        last_element.get_key(diffLevel) != current_element.get_key(diffLevel))
                    {
                        break;
                    }
                }

                // 2. Open all tags from diffLevel down to the current element's deepest level
                for (int j = diffLevel; j < current_no_levels; j++)
                {
                    string nm_element = schema.array_elem_names[current_element.get_position(j)];
                    output_xml.AppendLine($"{new string(' ', j * 2)}<{nm_element}>");
                }

                level = current_element.get_level();
                elemIndex = current_element.get_position(level);
                elemName = schema.array_elem_names[elemIndex];
                output_xml.AppendLine($"{new string(' ', level * 2)}<{elemName}>{current_element.txt_element}</{elemName}>");
            }
            foreach (int i in Enumerable.Range(1, current_no_levels))
            {
                string nm_element = schema.array_elem_names[current_element.get_position(current_no_levels - i)];
                output_xml.AppendLine($"{new string(' ', 2 * (current_no_levels - i))}</{nm_element}>");
            }
        }

        // Sorts elements by their position arrays for correct XML order.
        private void sort_elements_by_position()
        {
            elements.Sort((e1, e2) => ComparePositions(e1.position, e2.position));
        }

        // Compares two position arrays for sorting.
        private int ComparePositions(int[] pos1, int[] pos2)
        {
            if (pos1 == null && pos2 == null) return 0;
            if (pos1 == null) return -1;
            if (pos2 == null) return 1;

            int minLength = Math.Min(pos1.Length, pos2.Length);
            for (int i = 0; i < minLength; i++)
            {
                int comparison = pos1[i].CompareTo(pos2[i]);
                if (comparison != 0)
                {
                    return comparison;
                }
            }
            return pos1.Length.CompareTo(pos2.Length);
        }

        private static string XmlEscape(string value)
        {
            if (string.IsNullOrEmpty(value)) return string.Empty;

            // Quick check: if no special characters, return as-is
            if (value.IndexOfAny(new[] { '&', '<', '>', '"', '\'' }) == -1)
                return value;

            // Otherwise, do the escaping
            var sb = new StringBuilder(value.Length + 10);
            foreach (char c in value)
            {
                switch (c)
                {
                    case '&': sb.Append("&amp;"); break;
                    case '<': sb.Append("&lt;"); break;
                    case '>': sb.Append("&gt;"); break;
                    case '"': sb.Append("&quot;"); break;
                    case '\'': sb.Append("&apos;"); break;
                    default: sb.Append(c); break;
                }
            }
            return sb.ToString();
        }


    }

}// The dietXML class is a high-performance XML builder that constructs XML documents from flat data using a custom schema.
