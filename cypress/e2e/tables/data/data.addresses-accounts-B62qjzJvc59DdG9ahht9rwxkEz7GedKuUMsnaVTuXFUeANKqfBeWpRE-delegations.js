// Test suite data for: /addresses/accounts/B62qjzJvc59DdG9ahht9rwxkEz7GedKuUMsnaVTuXFUeANKqfBeWpRE/delegations
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
  url: "/addresses/accounts/B62qjzJvc59DdG9ahht9rwxkEz7GedKuUMsnaVTuXFUeANKqfBeWpRE/delegations",
  table: {
    heading: "Delegations",
    columns: ["Public Key", "Username", "Delegated Balance", "% of Delegation"],
    filter_tests: [],
  },
  tests: [],
};
