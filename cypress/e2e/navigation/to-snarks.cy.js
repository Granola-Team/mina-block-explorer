import { DEFAULT_ACCOUNT_PK } from "../constants";

suite(["@CI"], "snarks page", () => {
  [
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      dest: "snarks",
      href: `/snarks?account=${DEFAULT_ACCOUNT_PK}`,
    },
  ].forEach(({ origin, dest, href }) =>
    it(`is navigated to from ${dest}`, () => {
      cy.visit(origin);
      cy.get("a").contains("See all snark jobs").click();
      cy.url().should("contain", href);
    }),
  );
});
