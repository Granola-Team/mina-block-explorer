export const kebabCase = (string) =>
  string
    .replace(/([a-z])([A-Z])/g, "$1-$2")
    .replace(/[\s_]+/g, "-")
    .replace(/-+/g, "-")
    .toLowerCase();
export const removeParentheses = (str) => {
  return str.replace(/[()]/g, "");
};
export const parseFormattedNumber = (formattedString) => {
  let normalizedNumber = formattedString.replace(/[^0-9.]/g, "");
  return parseFloat(normalizedNumber);
};
