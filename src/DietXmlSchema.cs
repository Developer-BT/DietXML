using System.Collections.Generic;
using System.Text;



public class DietXmlSchema
{
    // this is used to parse the text schema definition provided by the user
    // it reads the schema and creates a list of elements with their positions in the schema
    // and does some basica validation of the schema (eg tags balance)

    internal string txt_schema;
    internal int level = 0;
    internal int element_tracker = 0;
    internal List<int> level_tracker = new List<int>();



    internal List<(string, List<int>)> list_elem_names = new List<(string, List<int>)>();
    internal string[] array_elem_names;

    internal Dictionary<string, (int[], int)> array_string_element_positions = new Dictionary<string, (int[], int)>(100);
    internal List<int>[] int_element_positions;






    internal int[] read_position(string nm_element)
    {
        return array_string_element_positions[nm_element].Item1;
    }





    internal int read_level(string nm_element)
    {
        return array_string_element_positions[nm_element].Item2 - 1;
    }

    internal void clear_key(string nm_element)
    {

    }



    /// <summary>
    /// Sets the XML schema definition for this builder and parses it to initialize element positions and hierarchy.
    /// </summary>
    /// <param name="txt_schema">The XML schema as a string. Must be well-formed and balanced.</param>

    internal void set_schema(string txt_schema)
    {
        this.txt_schema = txt_schema;
        this.read_schema();
    }

    private void read_schema()
    {
        StringBuilder buffer = new StringBuilder();
        bool ind_open = false;
        int openTags = 0;

        for (int idx = 0; idx < this.txt_schema.Length; idx++)
        {
            char i = this.txt_schema[idx];
            switch (i)
            {
                case '<':
                    if (ind_open)
                    {
                        throw new FormatException($"Unexpected '<' at position {idx}: Nested tag opening detected.");
                    }
                    ind_open = true;
                    break;
                case '>':
                    if (!ind_open)
                    {
                        throw new FormatException($"Unexpected '>' at position {idx}: No matching '<' found.");
                    }
                    ind_open = false;

                    if (buffer.Length == 0)
                    {
                        throw new FormatException($"Empty tag found at position {idx}.");
                    }

                    if (buffer[0] == '/')
                    {
                        // Closing tag
                        level--;
                        openTags--;
                        if (level < 0)
                        {
                            throw new FormatException($"Too many closing tags at position {idx}.");
                        }
                    }
                    else
                    {
                        // Opening tag
                        update_levels();
                        if (buffer.ToString().Any(char.IsWhiteSpace))
                        {
                            // throw new FormatException($"Invalid element name '{buffer}' at position {idx} (contains whitespace).");
                        }
                        record_element(buffer.ToString(), level_tracker);
                        level++;
                        openTags++;
                    }
                    buffer.Clear();
                    break;
                default:
                    if (ind_open)
                    {
                        buffer.Append(i);
                    }
                    break;
            }
        }

        if (ind_open)
        {
            throw new FormatException("Schema ended while inside a tag (missing '>').");
        }
        if (level != 0 || openTags != 0)
        {
            throw new FormatException("Unbalanced tags detected in schema.");
        }

        array_elem_names = list_elem_names.Select(item => item.Item1).ToArray();
    }
    private void record_element(string nm_element, List<int> level_tracker)
    {
        int[] new_pos = level_tracker.ToArray();
        array_string_element_positions.Add(nm_element, (new_pos, new_pos.Length));
        list_elem_names.Add((nm_element, new List<int>(new_pos)));
        // record element to element list
    }

    private void update_levels()
    {
        if (level + 1 > level_tracker.Count)
        {
            level_tracker.Add(element_tracker);
        }
        else
        {
            level_tracker[level] = element_tracker;
        }

        // Trim level_tracker to the current level
        if (level + 1 < level_tracker.Count)
        {
            level_tracker.RemoveRange(level + 1, level_tracker.Count - (level + 1));
        }

        element_tracker++;
    }


    internal void print_schema()
    {
        foreach (KeyValuePair<string, (int[], int)> i in array_string_element_positions)
        {
            Console.WriteLine($"{i.Key} {string.Join(", ", i.Value.Item1)}");
        }
    }


}
