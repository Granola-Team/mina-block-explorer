import {
  ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION,
  DEFAULT_ACCOUNT_PK,
  FIRST_BLOCK_PRODUCER_ADDRESS,
  FIRST_FEE_PAYER,
  FIRST_RECIPIENT_ADDRESS,
  FIRST_SENDER_ADDRESS,
} from "../constants";
import { kebabCase } from "../helpers";

let counterparty = "B62qjYanmV7y9njVeH5UHkz3GYBm7xKir1rAnoY4KsEYUGLMiU45FSM";
let state_hash = "3NKrxKGr3JpYT2CzAFUeUb89ae6MFMsVWFX1QLYqYNJp1ffHR4ej";

suite(["@tier2"], "search with multiple results", () => {
  let multi_response_searches = [
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      input: counterparty,
      tableHeading: "User Commands",
      expectation: { column: "Counterparty", value: counterparty },
    },
    {
      origin: `/addresses/accounts/${ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION}`,
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
    {
      origin: `/addresses/accounts/${ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION}`,
      input: "9963",
      tableHeading: "SNARK Jobs",
      expectation: { column: "Height", value: "9963" },
    },
    {
      origin: `/addresses/accounts/${ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION}`,
      input: "6165",
      tableHeading: "Block Production",
      expectation: { column: "Height", value: "6165" },
    },
    {
      origin: "/snarks",
      input: "921",
      tableHeading: "SNARKs",
      expectation: { column: "Height", value: "921" },
    },
    {
      origin: "/snarks",
      input: state_hash,
      tableHeading: "SNARKs",
      expectation: { column: "State Hash", value: state_hash },
    },
    {
      origin: "/snarks",
      input: "B62qqSvXBa1cdTsARTVZrnJreCDdEy162q7axsf3QfjWzZCKfrU6JoM",
      tableHeading: "SNARKs",
      expectation: {
        column: "Prover",
        value: "B62qqSvXBa1cdTsARTVZrnJreCDdEy162q7axsf3QfjWzZCKfrU6JoM",
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
