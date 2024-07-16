import { ROMEK_ADDRESS, WHISPERIT_BLOCK_STATE_HASH } from "../constants";

// TODO: unexplained errors in tier1; moving to tier2.
suite(["@tier2"], "transaction spotlight", () => {
  let pages = [
    {
      origin: `/blocks/accounts/${ROMEK_ADDRESS}`,
      column: "Hash",
      tableHeader: "User Commands",
      tableHeaderEl: "h2",
      transposed: true,
    },
    { origin: "/commands", column: "Hash", tableHeader: "User Commands" },
    {
      origin: `/blocks/${WHISPERIT_BLOCK_STATE_HASH}/commands/user`,
      column: "Hash",
      tableHeader: "User Commands",
    },
    {
      origin: `/addresses/accounts/${ROMEK_ADDRESS}`,
      column: "Txn Hash",
      tableHeader: "User Commands",
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
          cy.clickLinkInTable(1, column, tableHeader, tableHeaderEl);
        }
        cy.url().should("include", "/commands/");
      }),
  );
});
