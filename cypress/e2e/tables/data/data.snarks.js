// Test suite data for: /snarks
import { parseFormattedNumber } from "../../helpers";
import { BLOCK_WITH_ALL_ACTIVITY } from "../../constants";

export const url = "/snarks";
export const table = {
  heading: "SNARKs",
  columns: ["Height", "State Hash", "Date", "Prover", "Fee"],
  filter_tests: [
    {
      column: "Height",
      input: 360100,
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("SNARKs", 2);
        cy.assertForEachColumnValue("SNARKs", "Height", (text) => {
          let height = parseFormattedNumber(text);
          expect(height).to.be.lte(360100);
        });
      },
    },
    {
      column: "State Hash",
      input: BLOCK_WITH_ALL_ACTIVITY,
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("SNARKs", 2);
        cy.aliasTableRows("SNARKs", "table-rows");
        cy.get("@table-rows").should("have.length.greaterThan", 1);
        cy.assertForEachColumnValue("SNARKs", "State Hash", (text) => {
          expect(text).to.equal(BLOCK_WITH_ALL_ACTIVITY);
        });
      },
    },
    {
      column: "Prover",
      input: "B62qrQiw9JhUumq457sMxicgQ94Z1WD9JChzJu19kBE8Szb5T8tcUAC",
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("SNARKs", 2);
        cy.assertForEachColumnValue("SNARKs", "Prover", (text) => {
          expect(text).to.equal(
            "B62qrQiw9JhUumq457sMxicgQ94Z1WD9JChzJu19kBE8Szb5T8tcUAC",
          );
        });
      },
    },
  ],
};
export const tests = [
  [
    "has standard row limits",
    () => {
      cy.assertStandardRowLimits("SNARKs");
    },
  ],
  [
    "has working load next button",
    () => {
      cy.intercept("POST", "/graphql").as("graphql");
      cy.visit("/snarks?row-limit=50&q-height=359630");
      cy.wait("@graphql").then(() => {
        cy.wait(1000);
        cy.assertLoadNextWorks("SNARKs", "Height", {
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
