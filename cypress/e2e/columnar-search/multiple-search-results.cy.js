import {
  DEFAULT_ACCOUNT_PK,
  FIRST_BLOCK_PRODUCER_ADDRESS,
  FIRST_FEE_PAYER,
  FIRST_RECIPIENT_ADDRESS,
  FIRST_SENDER_ADDRESS,
} from "../constants";
import { kebabCase } from "../helpers";

let counterparty = "B62qjYanmV7y9njVeH5UHkz3GYBm7xKir1rAnoY4KsEYUGLMiU45FSM";
let state_hash = "3NKxUy4mRpuH7MJxFQEobEJbUhPyvDyMEBQywmTRLbWsaHto3nur";
let prover = "B62qs2Lw5WZNSjd8eHBUZXFYyRjV8oKtrZMFDn1S1Ye62G71xCQJMYM";

suite(["@CI"], "search with multiple results", () => {
  let multi_response_searches = [
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      input: FIRST_BLOCK_PRODUCER_ADDRESS,
      tableHeading: "Block Production",
      expectation: {
        column: "Block Producer",
        value: FIRST_BLOCK_PRODUCER_ADDRESS,
      },
    },
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      input: prover,
      tableHeading: "SNARK Jobs",
      expectation: { column: "Prover", value: prover },
    },
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      input: counterparty,
      tableHeading: "User Commands",
      expectation: { column: "Counterparty", value: counterparty },
    },
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      input: state_hash,
      tableHeading: "SNARK Jobs",
      expectation: { column: "State Hash", value: state_hash },
    },
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      input: "500",
      tableHeading: "User Commands",
      expectation: { column: "Height", value: "500" },
    },
    // TODO: re-enable these tests when searching by block height works on snarks
    // {
    //   origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
    //   input: "957",
    //   tableHeading: "SNARK Jobs",
    //   expectation: { column: "Height", value: "957" },
    // },
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      input: "500",
      tableHeading: "Block Production",
      expectation: { column: "Height", value: "500" },
    },
    {
      origin: "/next-stakes",
      input: DEFAULT_ACCOUNT_PK,
      tableHeading: "Next Staking Ledger",
      expectation: { column: "Delegate", value: DEFAULT_ACCOUNT_PK },
    },
    // TODO: re-enable these tests when searching by block height works on snarks
    // {
    //   origin: "/snarks",
    //   input: "940",
    //   tableHeading: "SNARKs",
    //   expectation: { column: "Height", value: "940" },
    // },
    {
      origin: "/snarks",
      input: state_hash,
      tableHeading: "SNARKs",
      expectation: { column: "State Hash", value: state_hash },
    },
    {
      origin: "/snarks",
      input: "B62qs2Lw5WZNSjd8eHBUZXFYyRjV8oKtrZMFDn1S1Ye62G71xCQJMYM",
      tableHeading: "SNARKs",
      expectation: {
        column: "Prover",
        value: "B62qs2Lw5WZNSjd8eHBUZXFYyRjV8oKtrZMFDn1S1Ye62G71xCQJMYM",
      },
    },
    {
      origin: "/commands/user",
      tableHeading: "User Commands",
      input: "50",
      expectation: { column: "Height", value: "50" },
    },
    {
      origin: "/commands/user",
      tableHeading: "User Commands",
      input: FIRST_SENDER_ADDRESS,
      expectation: { column: "From", value: FIRST_SENDER_ADDRESS },
    },
    {
      origin: "/commands/user",
      tableHeading: "User Commands",
      input: FIRST_RECIPIENT_ADDRESS,
      expectation: { column: "To", value: FIRST_RECIPIENT_ADDRESS },
    },
    {
      origin: "/commands/internal",
      input: FIRST_FEE_PAYER,
      tableHeading: "Internal Commands",
      expectation: { column: "Recipient", value: FIRST_FEE_PAYER },
    },
    {
      origin: "/commands/internal",
      input: "50",
      tableHeading: "Internal Commands",
      expectation: { column: "Height", value: "50" },
    },
    {
      origin: "/blocks",
      input: FIRST_BLOCK_PRODUCER_ADDRESS,
      tableHeading: "Blocks",
      expectation: {
        column: "Block Producer",
        value: FIRST_BLOCK_PRODUCER_ADDRESS,
      },
    },
    {
      origin: "/blocks",
      input: "50",
      tableHeading: "Blocks",
      expectation: { column: "Height", value: "50" },
    },
  ];

  multi_response_searches.forEach(
    ({ origin, input, tableHeading, expectation }) =>
      it(`works on ${origin} page when searching column '${expectation.column}'`, () => {
        let cssSelector = "#q-" + kebabCase(expectation.column);
        cy.visit(origin);
        cy.wait(1000);
        cy.get(cssSelector).type(input, { delay: 0 });
        cy.tableColumnValuesEqual(
          tableHeading,
          expectation.column,
          expectation.value,
        );
      }),
  );
});
