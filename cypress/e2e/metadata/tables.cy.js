import {
  TOTAL_NUM_FAILED_CANONICAL_USER_COMMANDS,
  TOTAL_NUMBER_USER_COMMANDS,
  TOTAL_NUM_APPLIED_CANONICAL_ZKAPP_COMMANDS,
  TOTAL_NUMBER_APPLIED_USER_COMMANDS,
  TOTAL_NUM_FAILED_CANONICAL_ZKAPP_COMMANDS,
  TOTAL_NUMBER_APPLIED_CANONICAL_USER_COMMANDS,
  TOTAL_NUM_FAILED_USER_COMMANDS,
  TOTAL_NUM_APPLIED_ZKAPP_COMMANDS,
  MINU_TOKEN_ADDRESS,
  TOTAL_NUM_MINU_TOKEN_TXN,
  TOTAL_NUM_FAILED_ZKAPP_COMMANDS,
  TOTAL_NUM_NFT_HOLDERS,
  NFT_TOKEN_ID,
  MINU_SYMBOL,
  TOTAL_NUM_CANONICAL_USER_COMMANDS,
} from "../constants";

const testMetadata = ({ url, table_header, metadata }) => {
  it(`is correct for ${url}`, () => {
    cy.visit(url);
    cy.waitUntilTableLoads(table_header);
    cy.wait(150);
    metadata.forEach((datum, ith) => {
      cy.assertTableMetadataCorrect(table_header, datum, ith);
    });
  });
};

describe("accounts metadata", () => {
  [
    {
      url: `/addresses/accounts?q-token=${NFT_TOKEN_ID}`,
      table_header: `NFT Token Accounts`,
      metadata: [1, TOTAL_NUM_NFT_HOLDERS],
    },
  ].forEach(testMetadata);
});

const row_limit = 25;
describe("user command metadata", () => {
  [
    {
      url: "/commands/user",
      table_header: "User Commands",
      metadata: [
        row_limit,
        TOTAL_NUMBER_APPLIED_CANONICAL_USER_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS,
      ],
    },
    {
      url: "/commands/user?q-status=All&txn-type=Canonical",
      table_header: "User Commands",
      metadata: [
        row_limit,
        TOTAL_NUM_CANONICAL_USER_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS,
      ],
    },
    {
      url: "/commands/user?q-status=All&txn-type=Non-Canonical",
      table_header: "User Commands",
      metadata: [
        row_limit,
        TOTAL_NUMBER_USER_COMMANDS - TOTAL_NUM_CANONICAL_USER_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS,
      ],
    },
    {
      url: "/commands/user?q-status=Applied&txn-type=Canonical",
      table_header: "User Commands",
      metadata: [
        row_limit,
        TOTAL_NUMBER_APPLIED_CANONICAL_USER_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS,
      ],
    },
    {
      url: "/commands/user?q-status=Failed&txn-type=Canonical",
      table_header: "User Commands",
      metadata: [
        TOTAL_NUM_FAILED_CANONICAL_USER_COMMANDS,
        TOTAL_NUM_FAILED_CANONICAL_USER_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS,
      ],
    },
    {
      url: "/commands/user?q-status=Applied&txn-type=Non-Canonical",
      table_header: "User Commands",
      metadata: [
        row_limit,
        TOTAL_NUMBER_APPLIED_USER_COMMANDS -
          TOTAL_NUMBER_APPLIED_CANONICAL_USER_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS,
      ],
    },
    {
      url: "/commands/user?q-status=Failed&txn-type=Non-Canonical",
      table_header: "User Commands",
      metadata: [
        TOTAL_NUM_FAILED_USER_COMMANDS -
          TOTAL_NUM_FAILED_CANONICAL_USER_COMMANDS,
        TOTAL_NUM_FAILED_USER_COMMANDS -
          TOTAL_NUM_FAILED_CANONICAL_USER_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS,
      ],
    },
    {
      url: "/commands/user?q-status=Applied&txn-type=Canonical&q-type=ZKAPP",
      table_header: "User Commands",
      metadata: [
        row_limit,
        TOTAL_NUM_APPLIED_CANONICAL_ZKAPP_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS,
      ],
    },
    {
      url: "/commands/user?q-status=Applied&txn-type=Non-Canonical&q-type=ZKAPP",
      table_header: "User Commands",
      metadata: [
        row_limit,
        TOTAL_NUM_APPLIED_ZKAPP_COMMANDS -
          TOTAL_NUM_APPLIED_CANONICAL_ZKAPP_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS,
      ],
    },
    {
      url: "/commands/user?q-status=Failed&txn-type=Canonical&q-type=ZKAPP",
      table_header: "User Commands",
      metadata: [
        TOTAL_NUM_FAILED_CANONICAL_ZKAPP_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS,
      ],
    },
    {
      url: "/commands/user?q-status=Failed&txn-type=Non-Canonical&q-type=ZKAPP",
      table_header: "User Commands",
      metadata: [
        TOTAL_NUM_FAILED_ZKAPP_COMMANDS -
          TOTAL_NUM_FAILED_CANONICAL_ZKAPP_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS,
      ],
    },
    {
      url: `/commands/user?q-token=${MINU_TOKEN_ADDRESS}`,
      table_header: `User Commands (${MINU_SYMBOL})`,
      metadata: [1, TOTAL_NUM_MINU_TOKEN_TXN, TOTAL_NUMBER_USER_COMMANDS],
    },
  ].forEach(testMetadata);
});

// TODO: Not a meaninful test due to blockchain length being greated than
// indexed value "total_num_blocks" when ingesting from hardfork
// describe( "blocks metadata", () => {
//   [
//     {
//       url: "/blocks",
//       table_header: "Blocks",
//       metadata: [
//         ...,
//         ...,
//         ...,
//       ],
//     },
//   ].forEach(testMetadata);
// });
