// Test suite data for: /blocks/3NKR3LoD3TLQ4Gotmhg59xUYcuq9hwVMuEujT99QywJhkDTYWwDy/commands/user
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
  url: "/blocks/3NKR3LoD3TLQ4Gotmhg59xUYcuq9hwVMuEujT99QywJhkDTYWwDy/commands/user",
  exclusive: true,
  table: {
    heading: "User Commands",
    columns: ["Hash", "Type", "Status", "From", "To", "Nonce", "Fee", "Amount"],
    filter_tests: [],
  },
  tests: [
    () => {
      cy.aliasTableRows("User Commands", "table-rows");
      cy.get("@table-rows").should("have.lengthOf", 6);
      cy.get("@table-rows").eq(0).should("contain.text", "Failed");
      cy.get("@table-rows").eq(5).should("contain.text", "Applied");
    },
  ],
};
