import { ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION } from "../constants";
suite(["@tier2"], "block page", () => {
  [
    {
      origin: `/addresses/accounts/${ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION}/block-production`,
      dest: "blocks",
      href: `/blocks?q-block-producer=${ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION}`,
    },
  ].forEach(({ origin, href }) =>
    it(`is navigated to from ${origin}`, () => {
      cy.visit(origin);
      cy.get("a").contains("See all block production").click();
      cy.url().should("contain", href);
      cy.tableColumnValuesEqual(
        "Blocks",
        "Block Producer",
        ADDRESS_WITH_SNARK_AND_BLOCK_PRODUCTION,
      );
    }),
  );
});
