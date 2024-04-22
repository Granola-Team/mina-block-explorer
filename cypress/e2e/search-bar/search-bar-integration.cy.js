import { DEFAULT_RECIPIENT } from "../constants";

suite(["@CI"], "search bar", () => {
  let state_hash = "CkpYfTKJyVjWmM5Lb5SdzRL6GuEbJf6q7yYAyW6NkvkYFZQaY5PGz";
  let block_hash = "3NLqPGGVtxXdsQg2orrp3SFFE3ToeMuqWRerSRWbmAKuSk2tphWy";
  let public_key = "B62qrQKS9ghd91shs73TCmBJRW9GzvTJK443DPx2YbqcyoLc56g1ny9";

  let pages = [
    { origin: "/", input: block_hash, tableHeading: "Blocks" },
    { origin: "/summary", input: block_hash, tableHeading: "Blocks" },
    { origin: "/blocks", input: block_hash, tableHeading: "Blocks" },
    {
      origin: "/commands/user-commands",
      input: state_hash,
      tableHeading: "User Commands",
    },
    {
      origin: "/stakes",
      input: public_key,
      tableHeading: "Current Staking Ledger",
    },
    {
      origin: "/next-stakes",
      input: public_key,
      tableHeading: "Next Staking Ledger",
    },
  ];

  it("works on /snarks page", () => {
    let prover = "B62qkwrHj3YCKgQsXRktpwhVFij19RiwYDgMmiwp7iggNBi8712a4W4";
    let tableHeading = "SNARKs";
    let tableColumn = "Prover";
    cy.visit("/snarks");
    cy.wait(1000);
    cy.get("input#searchbar").type(prover, { delay: 0 });
    cy.tableColumnValuesEqual(tableHeading, tableColumn, prover);
  });

  it("works on /transactions/internal-commands page", () => {
    let tableHeading = "Internal Commands";
    let tableColumn = "Recipient";
    cy.visit("/commands/internal-commands");
    cy.wait(1000);
    cy.get("input#searchbar").type(DEFAULT_RECIPIENT, { delay: 0 });
    cy.tableColumnValuesEqual(tableHeading, tableColumn, DEFAULT_RECIPIENT);
  });

  pages.forEach(({ origin, input, tableHeading }) =>
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
      cy.tableHasLessThanNRows(tableHeading, 5);
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
