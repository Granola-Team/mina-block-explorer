// Test suite data for: /staking-ledgers?epoch=1
const { SNZ_USERNAME, SNZPOOL_ADDRESS } = require("../../constants");

module.exports = {
  tag: "@tier2",
  url: "/staking-ledgers?epoch=1",
  table: {
    columns: [
      "Key",
      "Username",
      "Balance",
      "Stake",
      "Total Stake %",
      "Block Win %",
      "Delegate",
      "Delegators",
    ],
    sorting_columns: [
      {
        column: "Balance",
        type: "numeric",
        sort_options: ["BALANCE_DESC", "BALANCE_ASC"],
      },
      {
        column: "Total Stake %",
        type: "numeric",
        sort_options: [null, "STAKE_DESC", "STAKE_ASC"],
      },
    ],
    heading: "Staking Ledger - Epoch 1",
    filter_tests: [
      {
        column: "Key",
        input: "B62qq3tqfdj19hqaVCozJFM2q9gT2WezQMaJMKD6wxyvK3fMpHiP9va",
        assertion: function () {
          cy.aliasTableRows("Staking Ledger - Epoch 1", "table-rows");
          cy.get("@table-rows").should("have.lengthOf", 1);
          cy.assertForEachColumnValue(
            "Staking Ledger - Epoch 1",
            "Key",
            (text) => {
              expect(text).to.equal(SNZPOOL_ADDRESS);
            },
          );
          cy.tableColumnValuesEqual(
            "Staking Ledger - Epoch 1",
            "Username",
            SNZ_USERNAME,
          );
        },
      },
      {
        column: "Stake",
        input: "7,399,987.246422696",
        assertion: function () {
          cy.aliasTableRows("Staking Ledger - Epoch 1", "table-rows");
          cy.assertForEachColumnValue(
            "Staking Ledger - Epoch 1",
            "Stake",
            (text) => {
              expect(parseFloat(text)).to.be.lte(
                parseFloat("7,399,987.246422696"),
              );
            },
          );
        },
      },
    ],
  },
  tests: [
    () => {
      cy.get(".metadata")
        .invoke("text")
        .then((text) => {
          expect(text.split(" of ").length).to.equal(3);
        });
    },
    () => {
      cy.assertStandardRowLimits("Staking Ledger - Epoch 1");
    },
  ],
};