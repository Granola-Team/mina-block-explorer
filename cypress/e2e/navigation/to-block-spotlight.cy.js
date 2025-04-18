import { GENESIS_ACCOUNT_PK } from "../constants";
describe("block spotlight", () => {
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
      origin: `/addresses/accounts/${GENESIS_ACCOUNT_PK}/block-production`,
      selector: 'a[href^="/blocks/"]:not(a[href^="/blocks/account"])',
    },
  ].forEach(({ origin, selector }) =>
    it(`is navigated to from ${origin}`, () => {
      cy.visit(origin);
      cy.wait(1000);
      cy.get(selector).first().click({ force: true });
      cy.wait(1000);
      cy.url().should("include", "/blocks/");
    }),
  );
});
