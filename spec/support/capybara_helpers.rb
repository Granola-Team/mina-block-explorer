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
end
