// Test suite data for: /commands/user
import { parseFormattedNumber } from "../../helpers";
import { FIRST_RECIPIENT_ADDRESS, FIRST_SENDER_ADDRESS } from "../../constants";
export const tag = "@tier2";
export const url = "/commands/user";
export const table = {
  heading: "User Commands",
  columns: [
    "Height",
    "Txn Hash",
    "Date",
    "Type",
    "Status",
    "From",
    "To",
    "Nonce",
    "Fee",
    "Amount",
  ],
  filter_tests: [
    {
      column: "Height",
      input: 359613,
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("User Commands", 2);
        cy.assertForEachColumnValue("User Commands", "Height", (text) => {
          let height = parseFormattedNumber(text);
          expect(height).to.be.lte(359613);
        });
      },
    },
    {
      column: "Txn Hash",
      input: "5JvJnTKVwsxupNzXpRs5D3uQMsYSFE7NetN9o1KzbDAMjxFYziUg",
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("User Commands", 2);
        cy.aliasTableRows("User Commands", "table-rows");
        cy.get("@table-rows").should("have.lengthOf", 1);
        cy.assertForEachColumnValue("User Commands", "Txn Hash", (text) => {
          expect(text).to.contain(
            "5JvJnTKVwsxupNzXpRs5D3uQMsYSFE7NetN9o1KzbDAMjxFYziUg",
          );
          // expect(text).to.contain(ROMEK_NAMING_MEMO);
        });
      },
    },
    {
      column: "From",
      input: FIRST_SENDER_ADDRESS,
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("User Commands", 2);
        cy.assertForEachColumnValue("User Commands", "From", (text) => {
          expect(text).to.equal(FIRST_SENDER_ADDRESS);
        });
      },
    },
    {
      column: "To",
      input: FIRST_RECIPIENT_ADDRESS,
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("User Commands", 2);
        cy.assertForEachColumnValue("User Commands", "To", (text) => {
          expect(text).to.equal(FIRST_RECIPIENT_ADDRESS);
        });
      },
    },
  ],
};
export const tests = [
  () => {
    cy.assertStandardRowLimits("User Commands");
  },
  () => {
    cy.get("select#txn-status").as("txn-applied");
    cy.get("select#canonical-selection").as("canonical");
    ["Failed", "Applied"].forEach((txnApplied) => {
      ["Canonical"].forEach((canonical) => {
        cy.get("@txn-applied")
          .select(txnApplied)
          .should("have.value", txnApplied);
        cy.get("@canonical").select(canonical).should("have.value", canonical);
        cy.intercept("POST", "/graphql").as("graphql");
        cy.wait("@graphql").then(() => {
          cy.assertForEachColumnValue("User Commands", "Status", (text) => {
            expect(text).to.be.eq(txnApplied);
          });
          cy.intercept("POST", "/graphql").as("graphql2");
          cy.clickLinkInTable(0, "Txn Hash", "User Commands");
          cy.wait("@graphql2").then(() => {
            cy.testSpotlightValue("Status", txnApplied);
            cy.testSpotlightValue(
              "Canonical",
              "" + (canonical === "Canonical"),
            );
            cy.go("back");
          });
        });
      });
    });
  },
  () => {
    cy.getBySel("user-command-selection").as("select");

    // Select just zkApps
    cy.get("@select").select("zkApps").should("have.value", "zkApps");
    cy.wait(500); // small delay to let the request fire
    cy.waitUntilTableLoads("User Commands");
    cy.assertForEachColumnValue("User Commands", "Type", (text) => {
      expect(text).to.be.eq("Zkapp");
    });

    // Select all (both payments and zkapps)
    cy.get("@select").select("All").should("have.value", "All");
    cy.wait(500); // small delay to let the request fire
    cy.waitUntilTableLoads("User Commands");
    cy.assertForEachColumnValue("User Commands", "Type", (text) => {
      let result = text == "Zkapp" || text == "Payment";
      expect(result).to.be.true;
    });
  },
  () => {
    cy.intercept("POST", "/graphql").as("graphql");
    cy.visit("/commands/user?q-height=359611&row-limit=50");
    cy.wait("@graphql").then(() => {
      cy.wait(1000);
      cy.assertLoadNextWorks("User Commands", "Height", {
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
