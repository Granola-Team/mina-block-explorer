// Test suite data for: /commands/user
import { parseFormattedNumber } from "../../helpers";
import { FIRST_RECIPIENT_ADDRESS, FIRST_SENDER_ADDRESS } from "../../constants";

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
      column: "Type",
      input: "Zkapp",
      filter_type: "select",
      assertion: function () {
        cy.assertForEachColumnValue("User Commands", "Type", (text) => {
          expect(text).to.be.eq("Zkapp");
        });
      },
    },
    {
      column: "Type",
      input: "Payment",
      filter_type: "select",
      assertion: function () {
        cy.assertForEachColumnValue("User Commands", "Type", (text) => {
          expect(text).to.be.eq("Payment");
        });
      },
    },
    {
      column: "Type",
      input: "Stake Delegation",
      filter_type: "select",
      assertion: function () {
        cy.assertForEachColumnValue("User Commands", "Type", (text) => {
          expect(text).to.be.eq("Stake Delegation");
        });
      },
    },
    {
      column: "Status",
      input: "Applied",
      filter_type: "select",
      assertion: function () {
        cy.assertForEachColumnValue("User Commands", "Status", (text) => {
          expect(text).to.be.eq("Applied");
        });
      },
    },
    {
      column: "Status",
      input: "Failed",
      filter_type: "select",
      assertion: function () {
        cy.assertForEachColumnValue("User Commands", "Status", (text) => {
          expect(text).to.be.eq("Failed");
        });
      },
    },
    {
      column: "From",
      input: FIRST_SENDER_ADDRESS,
      assertion: function () {
        cy.assertForEachColumnValue("User Commands", "From", (text) => {
          expect(text).to.equal(FIRST_SENDER_ADDRESS);
        });
      },
    },
    {
      column: "To",
      input: FIRST_RECIPIENT_ADDRESS,
      assertion: function () {
        cy.assertForEachColumnValue("User Commands", "To", (text) => {
          expect(text).to.equal(FIRST_RECIPIENT_ADDRESS);
        });
      },
    },
  ],
};
export const tests = [
  [
    "has standard row limits",
    () => {
      cy.assertStandardRowLimits("User Commands");
    },
  ],
  [
    "has working load next button",
    () => {
      cy.visit("/commands/user?q-height=359611&row-limit=50");
      cy.waitUntilTableLoads("User Commands");
      cy.assertLoadNextWorks("User Commands", "Height", {
        button_text: "Load Next",
        expected_button_state: "be.disabled",
      });
    },
  ],
];
export default {
  url,
  table,
  tests,
};
