// Test suite data for: /commands/internal
import { parseFormattedNumber } from "../../helpers";
import { ROMEK_BLOCK_STATE_HASH } from "../../constants";
export const tag = "@tier2";
export const url = "/commands/internal";
export const table = {
  heading: "Internal Commands",
  columns: ["Height", "State Hash", "Recipient", "Fee", "Type", "Date"],
  filter_tests: [
    {
      column: "Height",
      input: 2000,
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("Internal Commands", 2);
        cy.assertForEachColumnValue("Internal Commands", "Height", (text) => {
          let height = parseFormattedNumber(text);
          expect(height).to.be.lte(2000);
        });
      },
    },
    {
      column: "State Hash",
      input: "3NLgCqncc6Ct4dcuhaG3ANQbfWwQCxMXu4MJjwGgRKxs6p8vQsZf",
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("Internal Commands", 2);
        cy.aliasTableRows("Internal Commands", "table-rows");
        cy.get("@table-rows").should("have.length.greaterThan", 1);
        cy.assertForEachColumnValue(
          "Internal Commands",
          "State Hash",
          (text) => {
            expect(text).to.equal(ROMEK_BLOCK_STATE_HASH);
          },
        );
      },
    },
    {
      column: "Recipient",
      input: "B62qnucUMHz7Dw2ReNgWhmR5XCvPeQjJWPReuQ8GwPyY4qj1otGBiKr",
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("Internal Commands", 2);
        cy.assertForEachColumnValue(
          "Internal Commands",
          "Recipient",
          (text) => {
            expect(text).to.equal(
              "B62qnucUMHz7Dw2ReNgWhmR5XCvPeQjJWPReuQ8GwPyY4qj1otGBiKr",
            );
          },
        );
      },
    },
  ],
};
export const tests = [
  () => {
    cy.assertStandardRowLimits("Internal Commands");
  },
  () => {
    cy.intercept("POST", "/graphql").as("graphql");
    cy.visit("/commands/internal?q-height=25");
    cy.wait("@graphql").then(() => {
      cy.wait(1000);
      cy.assertLoadNextWorks("Internal Commands", "Height", {
        button_text: "Load Next",
        expected_button_state: "be.disabled",
      });
    });
  },
];
export default {
  tag,
  url,
  table,
  tests,
};
