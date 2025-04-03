suite(["@tier2"], "accounts listing page", () => {
  let pages = [
    {
      origin: `/tokens`,
      column: "Holders",
      tableHeader: "Tokens",
    },
  ];
  pages.forEach(
    ({ origin, column, tableHeader, tableHeaderEl = "h1", transposed }) =>
      it(`is navigated to from ${origin} by clicking link in '${column}'`, () => {
        cy.visit(origin);
        if (transposed) {
          cy.clickLinkInTransposedTable(column, tableHeader, tableHeaderEl);
        } else {
          cy.clickLinkInTable(0, column, tableHeader, tableHeaderEl);
        }
        cy.url().should("include", "/addresses/accounts");
      }),
  );
});
