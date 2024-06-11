import { DEFAULT_ACCOUNT_PK } from "../constants";

suite(["@tier1"], "transaction spotlight", () => {
  let pages = [
    {
      origin:
        "/blocks/accounts/B62qqW8uKTxHZueKJwsoPY8NZcKVeDK4bLEHRkpMM2uKtEmmqLbkiQC",
      column: "Hash",
      tableHeader: "User Commands",
      tableHeaderEl: "h2",
      transposed: true,
    },
    { origin: "/commands", column: "Hash", tableHeader: "User Commands" },
    {
      origin:
        "/blocks/3NKnLbpRcFaY9WSzLFa4wYxejhnWBQNEPA2cnesvS75wcuNCr8nA/commands/user",
      column: "Hash",
      tableHeader: "User Commands",
    },
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      column: "Txn Hash",
      tableHeader: "User Commands",
    },
  ];

  pages.forEach(
    ({ origin, column, tableHeader, tableHeaderEl = "h1", transposed }) =>
      it(`is navigated to from ${origin} by clicking link in '${column}'`, () => {
        cy.visit(origin);
        if (transposed) {
          cy.clickLinkInTransposedTable(column, tableHeader, tableHeaderEl);
        } else {
          cy.clickLinkInTable(1, column, tableHeader, tableHeaderEl);
        }
        cy.url().should("include", "/commands/");
      }),
  );
});
