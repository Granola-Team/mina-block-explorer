# spec/support/table_helpers.rb
module TableHelpers
  def test_ordered_columns(heading, expected_columns)
    table = find("table[data-test='#{to_kebab_case(heading.downcase)}-table']")
    header_row = table.find("tr:has(th)")
    headers = header_row.all("th").map { |th| th.text.strip }
    expect(headers).to eq(expected_columns), "Expected table headers to be #{expected_columns}, but found #{headers}"
  end

  def test_valid_dates(heading)
    date_cells = all(table_column_selector(heading, "Date".upcase))
    date_cells.each do |cell|
      date_text = cell.text
      parsed_date = begin
        DateTime.parse(date_text)
      rescue
        nil
      end
      expect(parsed_date).not_to be_nil, "Expected '#{date_text}' to be a valid date, but it was not"
    end
  end

  def test_sortable_column(heading, column, sort_options)
    sort_options.each_with_index do |sort, i|
      if sort
        # Verify sort direction in URL (except for the first iteration)
        if i != 0
          expect(page.current_url).to include("sort-dir=#{sort}"), "Expected URL to include 'sort-dir=#{sort}', but was '#{page.current_url}'"
        end

        # Click the column to sort
        find("th", text: column.upcase).click
        wait_until_table_loaded(heading) # Wait for table to reload after sorting
      end
    end
  end

  def test_filter(heading, column, input, filter_type, assertion = nil)
    # Find the filter input/select
    filter_container = find("th", text: column.upcase)
    if filter_type == "select"
      filter_input = filter_container.find("select")
      filter_input.select(input)
    else
      filter_input = filter_container.find("input")
      filter_input.set(input)
    end

    sleep 1 # Wait for user input to trigger table load (equivalent to cy.wait(1000))
    wait_until_table_loaded(heading)

    # Run the provided assertion if given
    assertion&.call

    # Verify table records (equivalent to cy.assertTableRecordsCorrect)
    table_rows = get_table_rows(heading)
    expect(table_rows).not_to be_empty, "Expected table '#{heading}' to have records after filtering"
  end
end
