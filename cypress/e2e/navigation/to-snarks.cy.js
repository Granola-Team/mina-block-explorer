import { ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION } from "../constants";

suite(["@tier1"], "snarks page", () => {
  [
    {
      origin: `/addresses/accounts/${ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION}`,
      dest: "snarks",
      href: `/snarks?q-prover=${ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION}`,
    },
  ].forEach(({ origin, dest, href }) =>
    it(`is navigated to from ${dest}`, () => {
      cy.visit(origin);
      cy.get("a").contains("See all snark jobs").click();
      cy.url().should("contain", href);
      cy.tableColumnValuesEqual(
        "SNARKs",
        "Prover",
        ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION,
      );
    }),
  );
});
