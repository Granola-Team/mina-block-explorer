// Test suite data for: /commands/internal
import { parseFormattedNumber } from "../../helpers";

export const url = "/commands/internal";
export const table = {
  heading: "Internal Commands",
  columns: ["Height", "State Hash", "Recipient", "Fee", "Type", "Date"],
  filter_tests: [
    {
      column: "Height",
      input: 359610,
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("Internal Commands", 2);
        cy.assertForEachColumnValue("Internal Commands", "Height", (text) => {
          let height = parseFormattedNumber(text);
          expect(height).to.be.lte(359610);
        });
      },
    },
    {
      column: "State Hash",
      input: "3NLCZXPQH8WwSXUtrCHHz3n78RQJxNYUyn4uyx6JyTo4THApAbZd",
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("Internal Commands", 2);
        cy.aliasTableRows("Internal Commands", "table-rows");
        cy.get("@table-rows").should("have.length.greaterThan", 1);
        cy.assertForEachColumnValue(
          "Internal Commands",
          "State Hash",
          (text) => {
            expect(text).to.equal(
              "3NLCZXPQH8WwSXUtrCHHz3n78RQJxNYUyn4uyx6JyTo4THApAbZd",
            );
          },
        );
      },
    },
    {
      column: "Recipient",
      input: "B62qioAD9geuKsffk9gXSgHf18riNEB9NmR4Zyuo2fvWd5WWYTg4WHB",
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("Internal Commands", 2);
        cy.assertForEachColumnValue(
          "Internal Commands",
          "Recipient",
          (text) => {
            expect(text).to.equal(
              "B62qioAD9geuKsffk9gXSgHf18riNEB9NmR4Zyuo2fvWd5WWYTg4WHB",
            );
          },
        );
      },
    },
  ],
};
export const tests = [
  [
    "has standard row limits",
    () => {
      cy.assertStandardRowLimits("Internal Commands");
    },
  ],
  [
    "has working load next button",
    () => {
      cy.intercept("POST", "/graphql").as("graphql");
      cy.visit("/commands/internal?q-height=359618");
      cy.wait("@graphql").then(() => {
        cy.wait(1000);
        cy.assertLoadNextWorks("Internal Commands", "Height", {
          button_text: "Load Next",
          expected_button_state: "be.disabled",
        });
      });
    },
  ],
];
export default {
  url,
  table,
  tests,
};
