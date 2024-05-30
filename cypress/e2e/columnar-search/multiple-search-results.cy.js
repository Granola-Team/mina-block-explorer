import {
  DEFAULT_ACCOUNT_PK,
  FIRST_BLOCK_PRODUCER_ADDRESS,
  FIRST_FEE_PAYER,
  FIRST_RECIPIENT_ADDRESS,
  FIRST_SENDER_ADDRESS,
} from "../constants";
import { kebabCase } from "../helpers";

let counterparty = "B62qrrx8JKpWzZUq5kEc8Yh3qZqwUjTSr5wztmrPYJZRiowhZUZcs5g";
let prover = "B62qopzjbycAJDzvhc1tEuYSmJYfRQQbfS9nvkKtUzBS1fmLCyTz4dJ";
let block_producer = "B62qkgy1rQQmSL91aFeFvrYi9ptqavvgVkUiPZHmy5tZacSupTTCGi6";
let state_hash = "3NKypQg4LpXcWW2BPzue3e93eDKPHMpZ5J4jLNptVwuS7xDBDPzX";

suite(["@CI"], "search with multiple results", () => {
  let multi_response_searches = [
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      input: block_producer,
      tableHeading: "Block Production",
      expectation: { column: "Block Producer", value: block_producer },
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
      input: "253134",
      tableHeading: "User Commands",
      expectation: { column: "Height", value: "253134" },
    },
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      input: "253134",
      tableHeading: "SNARK Jobs",
      expectation: { column: "Height", value: "253134" },
    },
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      input: "253134",
      tableHeading: "Block Production",
      expectation: { column: "Height", value: "253134" },
    },
    {
      origin: "/next-stakes",
      input: DEFAULT_ACCOUNT_PK,
      tableHeading: "Next Staking Ledger",
      expectation: { column: "Delegate", value: DEFAULT_ACCOUNT_PK },
    },
    {
      origin: "/snarks",
      input: "350428",
      tableHeading: "SNARKs",
      expectation: { column: "Height", value: "350428" },
    },
    {
      origin: "/snarks",
      input: state_hash,
      tableHeading: "SNARKs",
      expectation: { column: "State Hash", value: state_hash },
    },
    {
      origin: "/snarks",
      input: DEFAULT_ACCOUNT_PK,
      tableHeading: "SNARKs",
      expectation: { column: "Prover", value: DEFAULT_ACCOUNT_PK },
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
