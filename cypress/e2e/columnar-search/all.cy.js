import { DEFAULT_ACCOUNT_PK } from "../constants";

const kebabCase = (string) =>
  string
    .replace(/([a-z])([A-Z])/g, "$1-$2")
    .replace(/[\s_]+/g, "-")
    .toLowerCase();

let state_hash = "3NKypQg4LpXcWW2BPzue3e93eDKPHMpZ5J4jLNptVwuS7xDBDPzX";
let counterparty = "B62qrrx8JKpWzZUq5kEc8Yh3qZqwUjTSr5wztmrPYJZRiowhZUZcs5g";
let prover = "B62qopzjbycAJDzvhc1tEuYSmJYfRQQbfS9nvkKtUzBS1fmLCyTz4dJ";

suite(["@CI"], "search with multiple results", () => {
  let multi_response_searches = [
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
      input: "1",
      tableHeading: "User Commands",
      expectation: { column: "Nonce", value: "1" },
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
      origin: "/commands/user-commands",
      tableHeading: "User Commands",
      input: "350137",
      expectation: { column: "Height", value: "350137" },
    },
    {
      origin: "/commands/user-commands",
      tableHeading: "User Commands",
      input: DEFAULT_ACCOUNT_PK,
      expectation: { column: "From", value: DEFAULT_ACCOUNT_PK },
    },
    {
      origin: "/commands/user-commands",
      tableHeading: "User Commands",
      input: DEFAULT_ACCOUNT_PK,
      expectation: { column: "To", value: DEFAULT_ACCOUNT_PK },
    },
    {
      origin: "/commands/internal-commands",
      input: DEFAULT_ACCOUNT_PK,
      tableHeading: "Internal Commands",
      expectation: { column: "Recipient", value: DEFAULT_ACCOUNT_PK },
    },
    {
      origin: "/",
      input: DEFAULT_ACCOUNT_PK,
      tableHeading: "Blocks",
      expectation: { column: "Block Producer", value: DEFAULT_ACCOUNT_PK },
    },
    {
      origin: "/summary",
      input: DEFAULT_ACCOUNT_PK,
      tableHeading: "Blocks",
      expectation: { column: "Block Producer", value: DEFAULT_ACCOUNT_PK },
    },
    {
      origin: "/blocks",
      input: DEFAULT_ACCOUNT_PK,
      tableHeading: "Blocks",
      expectation: { column: "Block Producer", value: DEFAULT_ACCOUNT_PK },
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

suite(["@CI"], "search with single result", () => {
  let block_hash = "3NLqPGGVtxXdsQg2orrp3SFFE3ToeMuqWRerSRWbmAKuSk2tphWy";

  let exact_searches = [
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      input: state_hash,
      tableHeading: "Block Production",
      column: "State Hash",
    },
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      input: "253134",
      tableHeading: "Block Production",
      column: "Height",
    },
    {
      origin: "/staking-ledgers",
      input: DEFAULT_ACCOUNT_PK,
      tableHeading: "Current Staking Ledger",
      column: "Key",
    },
    {
      origin: "/next-stakes",
      input: DEFAULT_ACCOUNT_PK,
      tableHeading: "Next Staking Ledger",
      column: "Key",
    },
    {
      origin: "/commands/user-commands",
      input: "CkpZuatq9Q4CC39FbMbJVZucBmzwyJySvWXGq3s3JtX5Wr2ccpMMN",
      tableHeading: "User Commands",
      column: "Txn Hash",
    },
    {
      origin: "/",
      input: block_hash,
      tableHeading: "Blocks",
      column: "State Hash",
    },
    {
      origin: "/summary",
      input: block_hash,
      tableHeading: "Blocks",
      column: "State Hash",
    },
    {
      origin: "/blocks",
      input: block_hash,
      tableHeading: "Blocks",
      column: "State Hash",
    },
    { origin: "/", input: "20345", tableHeading: "Blocks", column: "Height" },
    {
      origin: "/summary",
      input: "20345",
      tableHeading: "Blocks",
      column: "Height",
    },
    {
      origin: "/blocks",
      input: "20345",
      tableHeading: "Blocks",
      column: "Height",
    },
    // { origin: "/", input: "20345", tableHeading: "Blocks", column: "Slot" },
    // { origin: "/summary", input: "20345", tableHeading: "Blocks", column: "Slot" },
    // { origin: "/blocks", input: "20345", tableHeading: "Blocks", column: "Slot" },
  ];

  exact_searches.forEach(({ origin, input, tableHeading, column }) =>
    it(`works on ${origin} page when searching column '${column}'`, () => {
      /* 
        Sufficiently "tall" viewport to display many rows per table.
        We want to see that the search bar is filtering results.  
      */
      cy.viewport(768, 2000);
      cy.visit(origin);
      cy.wait(1000);
      let key = "q-" + kebabCase(column);
      let cssSelector = "#" + key;
      cy.get(cssSelector).as("searchinput");
      cy.get("@searchinput").type(input, { delay: 0 });

      // check input
      cy.get("@searchinput").should("have.value", input);
      // check url
      cy.url().should("include", `${key}=${input}`);
      // check table
      cy.aliasTableRows(tableHeading, "table-rows");
      cy.get("@table-rows").should("have.length", 1);
      cy.wait(1000);

      cy.go("back");

      // check input
      cy.get("@searchinput").should("have.value", "");
      // check url
      cy.url().should("not.contain", key);
      // check table
      cy.tableHasMoreThanNRows(tableHeading, 15);
    }),
  );
});
