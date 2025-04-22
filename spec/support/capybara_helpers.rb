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

  def wait_until_table_loaded(heading, wait: 1)
    table_selector = "[data-test='#{to_kebab_case(heading.downcase)}-table']"

    # Wait for placeholders to appear, if need be
    page.has_css?("#{table_selector} .loading-placeholder", wait: wait, visible: true)

    # Loop until placeholders are gone
    loop do
      break unless page.has_css?("#{table_selector} .loading-placeholder", wait: 0, visible: true)
    end

    # Final assertion to ensure placeholders are gone
    expect(page).not_to have_css("#{table_selector} .loading-placeholder", wait: 0, visible: true)
  end

  def table_column_selector(table_header, column_name)
    # Construct the table selector
    table_selector = "table[data-test='#{to_kebab_case(table_header.downcase)}-table']"

    # Find the table
    table = find(table_selector, wait: 1)

    # Find the header row and identify the column index
    header_row = table.find("tr:has(th)", wait: 1)
    headers = header_row.all("th")
    column_index = headers.index { |th| th.text.strip == column_name }
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
    link = target_cell.find("a", wait: 1)
    link.click
  end

  def get_table_metadata(table_header)
    metadata_selector = "metadata-#{to_kebab_case(table_header)}"
    metadata_element = get_by_sel(metadata_selector)
    metadata_text = metadata_element.text
    # Parse "x of y of z" format, removing commas for number conversion
    metadata_text.scan(/\d[\d,]*/).map { |num| num.delete(",").to_i }
  end
end
