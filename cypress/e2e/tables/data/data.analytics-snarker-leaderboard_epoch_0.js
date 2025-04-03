import { parseFormattedNumber } from "../../helpers.js";
import {
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
} from "../../constants.js";
export const tag = "@tier2";
export const url = "/analytics/snarker-leaderboard?epoch=0";
export const table = {
  heading: "Snarker Leaderboard",
  columns: [
    "Username",
    "Public Key",
    "Total Fees",
    "Min Fee",
    "Max Fee",
    "Snarks Sold",
  ],
  sorting_columns: [
    {
      column: "Max Fee",
      type: "numeric",
      sort_options: [null, "MAX_FEE_DESC", "MAX_FEE_ASC"],
    },
    {
      column: "Total Fees",
      type: "numeric",
      sort_options: [null, "TOTAL_FEES_DESC", "TOTAL_FEES_ASC"],
    },
  ],
  filter_tests: [],
};
export const tests = [];
export default {
  tag,
  url,
  table,
  tests,
};
