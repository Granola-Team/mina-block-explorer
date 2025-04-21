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

  def wait_until_table_loaded(heading)
    table_selector = "[data-test='#{to_kebab_case(heading)}-table']"
    # wait for the placeholder to appear in the table, for a max of 1 second
    # (this is better than using `sleep 1` as it might return quicker)
    page.has_css?("#{table_selector} .loading-placeholder", wait: 3)
    # Then assert the table does not have the placeholder
    expect(page).not_to have_css("#{table_selector} .loading-placeholder", wait: 60)
  end
end
