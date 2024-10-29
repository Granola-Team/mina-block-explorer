// Test suite data for: /analytics/staker-leaderboard?epoch=0
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
  url: "/analytics/staker-leaderboard?epoch=0",
  table: {
    heading: "Staker Leaderboard",
    columns: [
      "Username",
      "Public Key",
      "Canonical Blocks Produced",
      "Supercharged Blocks Produced",
      "Slots Produced",
      "Orphan Rate",
    ],
    filter_tests: [],
  },
  tests: [],
};
