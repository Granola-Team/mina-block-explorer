import { DEFAULT_ACCOUNT_PK } from "../constants";
suite(["@tier2"], "snarks page", () => {
  [
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}/snark-jobs`,
      dest: "snarks",
      href: `/snarks?q-prover=${DEFAULT_ACCOUNT_PK}`,
    },
  ].forEach(({ origin, dest, href }) =>
    it(`is navigated to from ${dest}`, () => {
      cy.visit(origin);
      cy.get("a").contains("See all snark jobs").click();
      cy.url().should("contain", href);
      cy.tableColumnValuesEqual("SNARKs", "Prover", DEFAULT_ACCOUNT_PK);
    }),
  );
});
