// Test suite data for: /addresses/accounts/B62qiVr4Wy6yKhxNV49Npnpr2XF5AhsFejFWWQpWKARQpTYsb9snNZY/commands/User
import { FIRST_SENDER_ADDRESS } from "../../constants";
import { parseFormattedNumber } from "../../helpers";

export const url = `/addresses/accounts/${FIRST_SENDER_ADDRESS}/commands/user`;
export const table = {
  heading: "User Commands",
  columns: [
    "Height",
    "Txn Hash",
    "Nonce",
    "Date",
    "Type",
    "Direction",
    "Counterparty",
    "Amount/Fee",
  ],
  filter_tests: [
    {
      column: "Height",
      input: 359900,
      assertion: function () {
        cy.wait(1000);
        cy.assertForEachColumnValue("User Commands", "Height", (text) => {
          let height = parseFormattedNumber(text);
          expect(height).to.be.lte(359900);
        });
      },
    },
    {
      column: "Txn Hash",
      input: "5JuqQth67hX432bpfrkpcA5ceayBQt8dBLZxYRnPbWBmLasryP3b",
      assertion: function () {
        cy.aliasTableRows("User Commands", "table-rows");
        cy.assertForEachColumnValue("User Commands", "Txn Hash", (text) => {
          expect(text).to.contain(
            "5JuqQth67hX432bpfrkpcA5ceayBQt8dBLZxYRnPbWBmLasryP3b",
          );
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
      column: "Direction",
      input: "In",
      filter_type: "select",
      assertion: function () {
        cy.assertForEachColumnValue("User Commands", "Direction", (text) => {
          expect(text).to.be.eq("IN");
        });
      },
    },
    {
      column: "Direction",
      input: "Out",
      filter_type: "select",
      assertion: function () {
        cy.assertForEachColumnValue("User Commands", "Direction", (text) => {
          expect(text).to.be.eq("OUT");
        });
      },
    },
  ],
};
export const tests = [];
export default {
  url,
  table,
  tests,
};
