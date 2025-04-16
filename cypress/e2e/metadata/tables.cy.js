import {
  TOTAL_NUM_FAILED_CANONICAL_USER_COMMANDS,
  TOTAL_NUMBER_USER_COMMANDS,
  TOTAL_NUM_APPLIED_CANONICAL_ZKAPP_COMMANDS,
  TOTAL_NUMBER_APPLIED_USER_COMMANDS,
  // TOTAL_NUM_FAILED_CANONICAL_ZKAPP_COMMANDS,
  TOTAL_NUMBER_APPLIED_CANONICAL_USER_COMMANDS,
  TOTAL_NUM_FAILED_USER_COMMANDS,
  TOTAL_NUM_APPLIED_ZKAPP_COMMANDS,
  MINU_TOKEN_ADDRESS,
  TOTAL_NUM_MINU_TOKEN_TXN,
} from "../constants";

const row_limit = 25;
suite(["@tier2"], "user command metadata", () => {
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
    // TODO: fix these two later
    // {
    //   url: "/commands/user?q-status=Failed&txn-type=Canonical&q-type=ZKAPP",
    //   table_header: "User Commands",
    //   metadata: [
    //     TOTAL_NUM_FAILED_CANONICAL_ZKAPP_COMMANDS,
    //     TOTAL_NUMBER_USER_COMMANDS,
    //   ],
    // },
    // {
    //   url: "/commands/user?q-status=Failed&txn-type=Non-Canonical&q-type=ZKAPP",
    //   table_header: "User Commands",
    //   metadata: [
    //     TOTAL_NUM_FAILED_CANONICAL_ZKAPP_COMMANDS,
    //     TOTAL_NUMBER_USER_COMMANDS,
    //   ],
    // },
    {
      url: `/commands/user?q-token=${MINU_TOKEN_ADDRESS}`,
      table_header: "User Commands",
      metadata: [1, TOTAL_NUM_MINU_TOKEN_TXN, TOTAL_NUMBER_USER_COMMANDS],
    },
  ].forEach(({ url, table_header, metadata }) => {
    it(`is correct for ${url}`, () => {
      cy.visit(url);
      cy.waitUntilTableLoads(table_header);
      metadata.forEach((datum, ith) => {
        cy.assertTableMetadataCorrect(table_header, datum, ith);
      });
    });
  });
});
