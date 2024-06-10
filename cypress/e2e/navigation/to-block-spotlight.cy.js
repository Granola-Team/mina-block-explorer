import { DEFAULT_ACCOUNT_PK } from "../constants";

suite(["tier1"], "block spotlight", () => {
  [
    {
      origin: `/commands/internal`,
      selector: 'a[href^="/blocks/"]',
    },
    {
      origin: "/blocks",
      selector: 'a[href^="/blocks/"]:not(a[href^="/blocks/account"])',
    },
    {
      origin: "/blocks",
      selector: 'a[href^="/blocks/"]:not(a[href^="/blocks/account"])',
    },
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      selector: 'a[href^="/blocks/"]:not(a[href^="/blocks/account"])',
    },
  ].forEach(({ origin, selector }) =>
    it(`is navigated to from ${origin}`, () => {
      cy.visit(origin);
      cy.wait(1000);
      cy.get(selector, { timeout: 10000 }).first().click({ force: true });
      cy.wait(1000);
      cy.url().should("include", "/blocks/", { timeout: 10000 });
    }),
  );
});
