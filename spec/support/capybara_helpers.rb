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

  def click_link_in_table_column(table_header, column_text, nth_row)
    # Find the table using the table header text
    table_selector = "table[data-test='#{to_kebab_case(table_header.downcase)}-table']"
    table = page.find(table_selector, wait: 1)

    # Find the header row and identify the column index
    header_row = table.find("tr:has(th)", wait: 1)
    headers = header_row.all("th")
    column_index = headers.index { |th| th.text.strip == column_text }
    raise "Column '#{column_text}' not found in table '#{table_header}'" unless column_index

    # Adjust for 1-based indexing (nth_row is 1-based, CSS is 1-based)
    # Find the nth data row (excluding header row)
    data_rows = table.all("tr:not(:has(th))", wait: 1)
    raise "Row #{nth_row} not found in table '#{table_header}' (only #{data_rows.count} data rows available)" if nth_row > data_rows.count || nth_row < 1

    target_row = data_rows[nth_row - 1] # Convert to 0-based index for array access

    # Find the cell in the specified column (column_index is 0-based, CSS nth-child is 1-based)
    cell = target_row.find("td:nth-child(#{column_index + 1})", wait: 1)

    # Find and click the link within the cell
    link = cell.find("a", wait: 1)
    link.click
  end
end
