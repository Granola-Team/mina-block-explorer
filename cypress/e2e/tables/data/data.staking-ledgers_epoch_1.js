// Test suite data for: /staking-ledgers?epoch=1
import { SNZ_USERNAME, SNZPOOL_ADDRESS } from "../../constants";
export const url = "/staking-ledgers?epoch=1";
export const table = {
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
      column: "Total Stake %",
      type: "numeric",
      sort_options: ["STAKE_DESC", "STAKE_ASC"],
    },
  ],
  heading: "Staking Ledger - Epoch 1",
  filter_tests: [
    {
      column: "Key",
      input: "B62qq3tqfdj19hqaVCozJFM2q9gT2WezQMaJMKD6wxyvK3fMpHiP9va",
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("Staking Ledger - Epoch 1", 2);
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
        cy.assertNumberOfTableMetadataDatum("Staking Ledger - Epoch 1", 2);
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
    {
      column: "Delegate",
      input: "B62qjCuPisQjLW7YkB22BR9KieSmUZTyApftqxsAuB3U21r3vj1YnaG",
      assertion: function () {
        cy.assertNumberOfTableMetadataDatum("Staking Ledger - Epoch 1", 2);
      },
    },
  ],
};
export const tests = [
  () => {
    cy.visit("/staking-ledgers?epoch=1");
    cy.wait(1000);
    cy.assertNumberOfTableMetadataDatum("Staking Ledger - Epoch 1", 3);
  },
  () => {
    cy.assertStandardRowLimits("Staking Ledger - Epoch 1");
  },
  // TODO: unable to effectively find the end of the "load next" button
  // () => {
  //   cy.intercept("POST", "/graphql").as("graphql");
  //   cy.visit("/staking-ledgers?epoch=1&q-stake=66000.0&row-limit=275");
  //   cy.wait("@graphql").then(() => {
  //     cy.wait(1000);
  //     cy.assertLoadNextWorks("Staking Ledger - Epoch 1", "Stake", {
  //       button_text: "Load Next",
  //       expected_button_state: "be.disabled",
  //     });
  //   });
  // },
];
export default {
  url,
  table,
  tests,
};
