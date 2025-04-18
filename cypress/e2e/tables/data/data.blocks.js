// Test suite data for: /blocks
import { parseFormattedNumber } from "../../helpers";
import {
  FIRST_BLOCK_PRODUCER_ADDRESS,
  GENESIS_BLOCK_BLOCK_HASH,
  SLOTS_PER_EPOCH,
} from "../../constants";

export const url = "/blocks";
export const table = {
  heading: "Blocks",
  columns: [
    "Height",
    "State Hash",
    "Slot",
    "Date",
    "Block Producer",
    "Coinbase",
    "User Commands",
    "SNARKs",
    "Coinbase Receiver",
  ],
  filter_tests: [
    {
      column: "Height",
      input: 359900,
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("Blocks", 2);
        cy.assertForEachColumnValue("Blocks", "Height", (text) => {
          let height = parseFormattedNumber(text);
          expect(height).to.be.lte(359900);
        });
      },
    },
    {
      column: "State Hash",
      input: GENESIS_BLOCK_BLOCK_HASH,
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("Blocks", 2);
        cy.aliasTableRows("Blocks", "table-rows");
        cy.get("@table-rows").should("have.lengthOf", 1);
        cy.assertForEachColumnValue("Blocks", "State Hash", (text) => {
          expect(text).to.equal(GENESIS_BLOCK_BLOCK_HASH);
        });
      },
    },
    {
      column: "Slot",
      input: 565000,
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("Blocks", 2);
        cy.assertForEachColumnValue("Blocks", "Slot", (text) => {
          let height = parseFormattedNumber(text);
          expect(height).to.be.lte(565000);
          expect(height).to.be.gt(SLOTS_PER_EPOCH);
        });
      },
    },
    {
      column: "Block Producer",
      input: "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("Blocks", 2);
        cy.assertForEachColumnValue("Blocks", "Block Producer", (text) => {
          expect(text).to.equal(FIRST_BLOCK_PRODUCER_ADDRESS);
        });
      },
    },
  ],
};
export const tests = [
  ["has standard row limits",() => {
    cy.assertStandardRowLimits("Blocks");
  }],
  ["has working canonical filter",() => {
    cy.get("select#canonical-selection").as("canonical");
    cy.get("@canonical").select("Canonical");
    cy.intercept("POST", "/graphql").as("graphql");
    cy.wait("@graphql").then(() => {
      cy.aliasTableRows("Blocks", "table-rows");
      cy.get("@table-rows").find(".non-canonical").should("not.exist");
      cy.get("@table-rows").find(".canonical").should("exist");
    });
    cy.get("@canonical").select("Non-Canonical");
    cy.intercept("POST", "/graphql").as("graphql");
    cy.wait("@graphql").then(() => {
      cy.aliasTableRows("Blocks", "table-rows");
      cy.get("@table-rows").find(".non-canonical").should("exist");
      cy.get("@table-rows").find(".canonical").should("not.exist");
    });
    cy.get("@canonical").select("All");
    cy.intercept("POST", "/graphql").as("graphql");
    cy.wait("@graphql").then(() => {
      cy.aliasTableRows("Blocks", "table-rows");
      cy.get("@table-rows").find(".non-canonical").should("exist");
      cy.get("@table-rows").find(".canonical").should("exist");
    });
  }],
  ["has working load next button",() => {
    cy.intercept("POST", "/graphql").as("graphql");
    cy.visit("/blocks?q-height=359613");
    cy.wait("@graphql").then(() => {
      cy.wait(1000);
      cy.assertLoadNextWorks("Blocks", "Height", {
        button_text: "Load Next",
        expected_button_state: "be.disabled",
      });
    });
  }],
  ["has user command and zk txn counts in the user command column",() => {
    cy.get("th").contains("Height").find("input").as("input");
    cy.get("@input").clear();
    cy.get("@input").type("360580", { delay: 0 });
    cy.waitUntilTableLoads("Blocks");
    cy.aliasTableRows("Blocks", "table-rows");
    cy.get("@table-rows").first().should("contain", "58/2");
  }],
];
export default {
  url,
  table,
  tests,
};
