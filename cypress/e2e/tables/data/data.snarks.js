// Test suite data for: /snarks
const { parseFormattedNumber } = require("../../helpers");
const {
  FIRST_BLOCK_PRODUCER_ADDRESS,
  FIRST_RECIPIENT_ADDRESS,
  FIRST_SENDER_ADDRESS,
  GENESIS_BLOCK_BLOCK_HASH,
  BLOCK_STATE_HASH_MIXED_USER_COMMANDS,
  ROMEK_ADDRESS,
  ROMEK_MINA_NAMING_SERVICE_TXN_HASH,
  ROMEK_USERNAME,
  SLOTS_PER_EPOCH,
  MINA_NAMING_SERVICE_ADDRESS,
  ROMEK_BLOCK_STATE_HASH,
  VETAL_BLOCK_STATE_HASH,
  ROMEK_NAMING_MEMO,
  SNZ_USERNAME,
  SNZPOOL_ADDRESS,
} = require("../../constants");

module.exports = {
  tag: "@tier2",
  url: "/snarks",
  table: {
    heading: "SNARKs",
    columns: ["Height", "State Hash", "Date", "Prover", "Fee"],
    filter_tests: [
      {
        column: "Height",
        input: 2000,
        assertion: function () {
          cy.assertForEachColumnValue("SNARKs", "Height", (text) => {
            let height = parseFormattedNumber(text);
            expect(height).to.be.lte(2000);
          });
        },
      },
      {
        column: "State Hash",
        input: "3NKrxKGr3JpYT2CzAFUeUb89ae6MFMsVWFX1QLYqYNJp1ffHR4ej",
        assertion: function () {
          cy.aliasTableRows("SNARKs", "table-rows");
          cy.get("@table-rows").should("have.length.greaterThan", 1);
          cy.assertForEachColumnValue("SNARKs", "State Hash", (text) => {
            expect(text).to.equal(VETAL_BLOCK_STATE_HASH);
          });
        },
      },
      {
        column: "Prover",
        input: "B62qrQiw9JhUumq457sMxicgQ94Z1WD9JChzJu19kBE8Szb5T8tcUAC",
        assertion: function () {
          cy.assertForEachColumnValue("SNARKs", "Prover", (text) => {
            expect(text).to.equal(
              "B62qrQiw9JhUumq457sMxicgQ94Z1WD9JChzJu19kBE8Szb5T8tcUAC",
            );
          });
        },
      },
    ],
  },
  tests: [
    () => {
      cy.assertStandardRowLimits("SNARKs");
    },
  ],
};
