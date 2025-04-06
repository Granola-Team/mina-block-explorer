import { GENESIS_ACCOUNT_PK, APPLIED_TXN_BLOCK_STATE_HASH } from "../constants";
suite(["@tier2"], "transaction spotlight", () => {
  let pages = [
    { origin: "/commands", column: "Hash", tableHeader: "User Commands" },
    {
      origin: `/blocks/${APPLIED_TXN_BLOCK_STATE_HASH}/commands/user`,
      column: "Hash",
      tableHeader: "User Commands",
    },
    {
      origin: `/addresses/accounts/${GENESIS_ACCOUNT_PK}`,
      column: "Txn Hash",
      tableHeader: "User Commands",
    },
    {
      origin: `/tokens`,
      column: "Transactions",
      tableHeader: "Tokens",
    },
  ];
  pages.forEach(
    ({ origin, column, tableHeader, tableHeaderEl = "h1", transposed }) =>
      it(`is navigated to from ${origin} by clicking link in '${column}'`, () => {
        cy.visit(origin);
        cy.wait(100);
        cy.get(".loading-placeholder").should("not.exist");
        if (transposed) {
          cy.clickLinkInTransposedTable(column, tableHeader, tableHeaderEl);
        } else {
          cy.clickLinkInTable(0, column, tableHeader, tableHeaderEl);
        }
        cy.url().should("include", "/commands/");
      }),
  );
});

suite(["@tier2"], "user commands page", () => {
  [
    {
      origin: `/addresses/accounts/${GENESIS_ACCOUNT_PK}/user-commands`,
      dest: "user commands",
      href: `/commands/user?q-to=${GENESIS_ACCOUNT_PK}`,
    },
  ].forEach(({ origin, dest, href }) =>
    it(`is navigated to from ${dest}`, () => {
      cy.visit(origin);
      cy.contains("See all user commands").click();
      cy.url().should("contain", href);
      cy.tableColumnValuesEqual("User Commands", "To", GENESIS_ACCOUNT_PK);
    }),
  );
});
