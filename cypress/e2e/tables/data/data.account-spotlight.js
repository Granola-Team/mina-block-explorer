// Test suite data for: /addresses/accounts/B62qiVr4Wy6yKhxNV49Npnpr2XF5AhsFejFWWQpWKARQpTYsb9snNZY/commands/internal
import { DEFAULT_ACCOUNT_PK } from "../../constants";
import { parseFormattedNumber } from "../../helpers";
export const tag = "@tier2";
export const url = `/addresses/accounts/${DEFAULT_ACCOUNT_PK}/commands/internal`;
export const table = {
  heading: "Internal Commands",
  columns: ["Height", "State Hash", "Fee", "Type", "Date"],
  filter_tests: [
    {
      column: "Height",
      input: 359900,
      assertion: function () {
        cy.wait(1000);
        cy.assertForEachColumnValue("Internal Commands", "Height", (text) => {
          let height = parseFormattedNumber(text);
          expect(height).to.be.lte(359900);
        });
      },
    },
    {
      column: "State Hash",
      input: "3NKgJBsyECQga3PSKvJRSWq1we8GgE4gawMTZv4eH6ebk8ZTxL34",
      assertion: function () {
        cy.aliasTableRows("Internal Commands", "table-rows");
        cy.assertForEachColumnValue(
          "Internal Commands",
          "State Hash",
          (text) => {
            expect(text).to.contain(
              "3NKgJBsyECQga3PSKvJRSWq1we8GgE4gawMTZv4eH6ebk8ZTxL34",
            );
          },
        );
      },
    },
  ],
};
export const tests = [];
export default {
  tag,
  url,
  table,
  tests,
};
