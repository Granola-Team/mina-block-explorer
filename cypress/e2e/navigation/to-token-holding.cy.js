//
import { TOKEN_ACTIVITY_ONLY_ADDRESS } from "../constants";
suite(["@tier2"], "token holding page", () => {
  let pages = [
    {
      origin: `/addresses/accounts/${TOKEN_ACTIVITY_ONLY_ADDRESS}/tokens`,
      column: "ID",
      tableHeader: "Tokens",
      nthRow: 0,
    },
  ];
  pages.forEach(({ origin, column, tableHeader, nthRow }) =>
    it(`is navigated to from ${origin} by clicking link in '${column}'`, () => {
      cy.visit(origin);
      cy.waitUntilTableLoads(tableHeader);
      cy.clickLinkInTable(nthRow, column, tableHeader);
      cy.url().should("match", new RegExp(`.*\\/tokens\\/.{47}.*`));
      cy.contains("Token Holding").should("exist");
    }),
  );
});
