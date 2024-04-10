import { DEFAULT_ACCOUNT_PK } from "../constants";

suite([], "transaction spotlight", () => {
  let pages = [
    {
      origin:
        "/summary/accounts/B62qqW8uKTxHZueKJwsoPY8NZcKVeDK4bLEHRkpMM2uKtEmmqLbkiQC",
      column: "Hash",
      tableHeader: "Transactions",
      tableHeaderEl: "h2",
      transposed: true,
    },
    { origin: "/transactions", column: "Hash", tableHeader: "Transactions" },
    {
      origin:
        "/blocks/3NKnLbpRcFaY9WSzLFa4wYxejhnWBQNEPA2cnesvS75wcuNCr8nA/user-commands",
      column: "Hash",
      tableHeader: "User Commands",
    },
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      column: "State Hash",
      tableHeader: "Transactions",
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
        cy.url().should("include", "/transactions/");
      }),
  );
});
