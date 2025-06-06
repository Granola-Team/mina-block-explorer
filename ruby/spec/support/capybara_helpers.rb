# spec/support/capybara_helpers.rb
require "benchmark"

module CapybaraHelpers
  def get_by_sel(selector)
    find("[data-test='#{selector}']")
  end

  def get_all_by_sel(selector)
    all("[data-test='#{selector}']")
  end

  def is_numeric?(element)
    text = element.text
    clean_text = text.delete(",")
    numeric_value = clean_text.to_f
    # puts "Numeric Value is: { original: '#{text}', cleaned: '#{clean_text}', parsed: #{numeric_value}, is_nan: #{numeric_value.nan?}, is_finite: #{numeric_value.finite?} }"
    expect(!numeric_value.nan? && numeric_value.finite?).to be true
  end

  def remove_parentheses(string)
    string.to_s.gsub(/[()]/, " ").strip.squeeze(" ")
  end

  def to_kebab_case(string)
    string
      .to_s                                   # Ensure input is a string
      .gsub(/([A-Z]+)([A-Z][a-z])/, '\1-\2')  # Separate camelCase (e.g., CamelCase -> Camel-Case)
      .gsub(/([a-z\d])([A-Z])/, '\1-\2')      # Separate CamelCase (e.g., camelCase -> camel-Case)
      .gsub(/[^a-zA-Z0-9\s]/, " ")            # Replace special characters (e.g., ?, =) with spaces
      .strip                                  # Remove leading/trailing spaces
      .gsub(/\s+/, "-")                       # Convert spaces to hyphens
      .downcase.squeeze("-")                  # Collapse multiple hyphens
  end

  def wait_until_table_loaded(heading, timeout: 60)
    # Initial wait of 1 second to ensure the loading placeholder has time to render in the UI
    sleep 1

    # Loop up to the timeout to check for absence of .loading-placeholder
    start_time = Time.now
    loop do
      table_rows = get_table_rows(heading)
      loading_rows = table_rows.filter { |row| row.has_css?(".loading-placeholder", wait: 0) }

      # Break if there are no loading placeholders
      if loading_rows.empty?
        break
      end

      # Check if timeout has been reached
      elapsed_time = Time.now - start_time
      if elapsed_time >= timeout
        puts "Timeout reached: Table '#{heading}' not loaded after #{elapsed_time.round(2)} seconds"
        raise "Timeout waiting for table '#{heading}' to load: found #{loading_rows.count} rows with .loading-placeholder"
      end

      sleep 0.5
    end

    # Final assertion to ensure no loading placeholders remain
    table_rows = get_table_rows(heading)
    loading_rows = table_rows.filter { |row| row.has_css?(".loading-placeholder", wait: 0) }
    expect(loading_rows).to be_empty, "Expected table '#{heading}' to have no rows with .loading-placeholder, but found #{loading_rows.count}"
  end

  def wait_until_spotlight_loaded(wait: 5)
    # Wait for placeholders to appear, if needed
    page.has_css?("section#spotlight-section .loading-placeholder", wait: wait, visible: true)

    # Loop until placeholders are gone
    loop do
      break unless page.has_css?("section#spotlight-section .loading-placeholder", wait: 0, visible: true)
    end

    # Final assertion to ensure placeholders are gone
    expect(page).not_to have_css("section#spotlight-section .loading-placeholder", wait: 0, visible: true)
  end

  def get_table_rows(heading, transposed = false)
    if transposed
      find_all("table[data-test='#{to_kebab_case(heading.downcase)}-table'] tr:has(th)")
    else
      find_all("table[data-test='#{to_kebab_case(heading.downcase)}-table'] tr:not(:has(th))")
    end
  end

  def tab_selector(text)
    "[data-test='#{to_kebab_case(text.downcase)}-tab']"
  end

  def remove_select(th)
    page.execute_script(<<-JS, th)
        const thElement = arguments[0];
        const selectElement = thElement.querySelector('select');
        if (selectElement) {
          selectElement.remove();
        }
    JS
  end

  def table_column_selector(table_header, column_name)
    # Construct the table selector
    table_selector = "table[data-test='#{to_kebab_case(table_header.downcase)}-table']"

    # Find the table
    table = find(table_selector, wait: 1)

    # Find the header row and identify the column index
    header_row = table.find("tr:has(th)", wait: 1)
    headers = header_row.all("th")
    cleaned_headers = headers.map do |th|
      remove_select(th)

      # Extract the remaining text from the <th>
      th.text.gsub(/\s+/, " ").strip
    end
    column_index = cleaned_headers.index { |txt| txt == column_name }
    raise "Column '#{column_name}' not found in table '#{table_header}'" unless column_index

    # Return selector for td elements in the specified column (excluding header row)
    "#{table_selector} tr:not(:has(th)) td:nth-child(#{column_index + 1})"
  end

  def click_link_in_table_column(table_header, column_text, nth_row)
    # Construct the selector for the specified column
    column_selector = table_column_selector(table_header, column_text)

    # Find all data rows in the specified column
    data_cells = all(column_selector, wait: 1)
    raise "Row #{nth_row} not found in table '#{table_header}' (only #{data_cells.count} data rows available)" if nth_row > data_cells.count || nth_row < 1

    # Select the nth cell (nth_row is 1-based, array index is 0-based)
    target_cell = data_cells[nth_row - 1]

    # Find and click the link within the cell
    link = target_cell.find_all("a", wait: 1).first
    link.click
  end

  def get_table_metadata(table_header)
    metadata_selector = "metadata-#{to_kebab_case(table_header)}"
    metadata_element = get_by_sel(metadata_selector)
    metadata_text = metadata_element.text
    # Parse "x of y of z" format, removing commas for number conversion
    metadata_text.scan(/\d[\d,]*/).map { |num| num.delete(",").to_i }
  end

  def click_nav_menu_item(lineage)
    raise ArgumentError, "Lineage must be a non-empty array" if !lineage.is_a?(Array) || lineage.empty?

    # Iterate through all but the last item to hover
    lineage[0..-2].each do |item|
      element = find("nav a", text: item.upcase, wait: 0)
      element.hover
    end

    # Click the last item
    find("nav a", text: lineage.last.upcase, wait: 0).click
  end

  # spec/support/test_helpers.rb
  def test_spotlight(heading, id, expected_values)
    # Verify the heading in section#spotlight-section h1
    expect(page).to have_selector("section#spotlight-section h1", text: heading), "Expected heading '#{heading}' in section#spotlight-section h1"

    # Verify the ID in #spotlight-id
    expect(page).to have_selector("#spotlight-id", text: id), "Expected ID '#{id}' in #spotlight-id"

    # Within the spotlight section table, verify each field and its corresponding value
    within("section#spotlight-section table") do
      # Get all table rows
      spotlight_rows = all("tr", wait: 0)

      expected_values.each do |field, expected_value|
        # Find the row containing the field in <th>
        row = spotlight_rows.find do |row|
          th = row.first("th", wait: 0, visible: false) # Use first to avoid ambiguity
          th && th.text.strip == field # Exact match after stripping whitespace
        end

        # check exists
        expect(row).not_to be_nil, "Expected to find a row with field '#{field}' in spotlight section table"

        # Verify the field exists in <th>
        expect(row).to have_selector("th", text: field, wait: 0), "Expected field '#{field}' in spotlight section table headers"

        # Verify the corresponding value in <td>
        actual_value = row.find("td", wait: 0).text.gsub(/[\n+-]/, "")
        expect(actual_value).to include(expected_value), "Expected value '#{expected_value}' for field '#{field}', but found '#{actual_value}'"
      end
    end
  end
end
