import { DEFAULT_ACCOUNT_PK, DEFAULT_RECIPIENT } from "../constants";

suite(["@CI"], "search with multiple results", () => {
  let multi_response_searches = [
    {
      origin: "/snarks",
      input: "B62qkwrHj3YCKgQsXRktpwhVFij19RiwYDgMmiwp7iggNBi8712a4W4",
      tableHeading: "SNARKs",
      expectation: {
        column: "Prover",
        value: "B62qkwrHj3YCKgQsXRktpwhVFij19RiwYDgMmiwp7iggNBi8712a4W4",
      },
    },
    {
      origin: "/commands/internal-commands",
      input: DEFAULT_RECIPIENT,
      tableHeading: "Internal Commands",
      expectation: { column: "Recipient", value: DEFAULT_RECIPIENT },
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
      it(`works on ${origin} page`, () => {
        cy.visit(origin);
        cy.wait(1000);
        cy.get("input#searchbar").type(input, { delay: 0 });
        cy.tableColumnValuesEqual(
          tableHeading,
          expectation.column,
          expectation.value,
        );
      }),
  );
});

suite(["@CI"], "search with single result", () => {
  let state_hash = "CkpYfTKJyVjWmM5Lb5SdzRL6GuEbJf6q7yYAyW6NkvkYFZQaY5PGz";
  let block_hash = "3NLqPGGVtxXdsQg2orrp3SFFE3ToeMuqWRerSRWbmAKuSk2tphWy";
  let public_key = "B62qrQKS9ghd91shs73TCmBJRW9GzvTJK443DPx2YbqcyoLc56g1ny9";

  let exact_searches = [
    { origin: "/", input: block_hash, tableHeading: "Blocks" },
    { origin: "/", input: "H132", tableHeading: "Blocks" },
    { origin: "/summary", input: block_hash, tableHeading: "Blocks" },
    { origin: "/summary", input: "H34780", tableHeading: "Blocks" },
    { origin: "/blocks", input: block_hash, tableHeading: "Blocks" },
    { origin: "/blocks", input: "H20345", tableHeading: "Blocks" },
    {
      origin: "/commands/user-commands",
      input: state_hash,
      tableHeading: "User Commands",
    },
    {
      origin: "/staking-ledgers",
      input: public_key,
      tableHeading: "Current Staking Ledger",
    },
    {
      origin: "/next-stakes",
      input: public_key,
      tableHeading: "Next Staking Ledger",
    },
  ];

  exact_searches.forEach(({ origin, input, tableHeading, expectation }) =>
    it(`works on ${origin} page`, () => {
      /* 
        Sufficiently "tall" viewport to display many rows per table.
        We want to see that the search bar is filtering results.  
      */
      cy.viewport(768, 2000);
      cy.visit(origin);
      cy.wait(1000);
      cy.get("input#searchbar").as("searchinput");
      cy.get("@searchinput").type(input, { delay: 0 });

      // check input
      cy.get("@searchinput").should("have.value", input);
      // check url
      cy.url().should("include", `query=${input}`);
      // check table
      cy.aliasTableRows(tableHeading, "table-rows");
      cy.get("@table-rows").should("have.length", 1);
      cy.wait(1000);

      cy.go("back");

      // check input
      cy.get("@searchinput").should("have.value", "");
      // check url
      cy.url().should("not.contain", `query`);
      // check table
      cy.tableHasMoreThanNRows(tableHeading, 15);
    }),
  );
});
