// Test suite data for: /addresses/accounts/B62qiVr4Wy6yKhxNV49Npnpr2XF5AhsFejFWWQpWKARQpTYsb9snNZY/commands/internal
import { parseFormattedNumber } from "../../helpers";

module.exports = {
  tag: "@tier2",
  url: "/addresses/accounts/B62qiVr4Wy6yKhxNV49Npnpr2XF5AhsFejFWWQpWKARQpTYsb9snNZY/commands/internal",
  table: {
    heading: "Internal Commands",
    columns: ["Height", "State Hash", "Fee", "Type", "Date"],
    filter_tests: [
      {
        column: "Height",
        input: 5200,
        assertion: function () {
          cy.wait(1000);
          cy.assertForEachColumnValue("Internal Commands", "Height", (text) => {
            let height = parseFormattedNumber(text);
            expect(height).to.be.lte(5200);
          });
        },
      },
      {
        column: "State Hash",
        input: "3NKq6mHhx31ikA9Gax1JcRuzTMp3tMudKfcwt3VxMDnvAeMYZGPA",
        assertion: function () {
          cy.aliasTableRows("Internal Commands", "table-rows");
          cy.assertForEachColumnValue(
            "Internal Commands",
            "State Hash",
            (text) => {
              expect(text).to.contain(
                "3NKq6mHhx31ikA9Gax1JcRuzTMp3tMudKfcwt3VxMDnvAeMYZGPA",
              );
            },
          );
        },
      },
    ],
  },
  tests: [],
};
