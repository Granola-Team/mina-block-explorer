import { FIRST_BLOCK_PRODUCER_ADDRESS } from "../constants";
describe("block page", () => {
  [
    {
      origin: `/addresses/accounts/${FIRST_BLOCK_PRODUCER_ADDRESS}/block-production`,
      dest: "blocks",
      href: `/blocks?q-block-producer=${FIRST_BLOCK_PRODUCER_ADDRESS}`,
    },
  ].forEach(({ origin, href }) =>
    it(`is navigated to from ${origin}`, () => {
      cy.visit(origin);
      cy.get("a").contains("See all block production").click();
      cy.url().should("contain", href);
      cy.tableColumnValuesEqual(
        "Blocks",
        "Block Producer",
        FIRST_BLOCK_PRODUCER_ADDRESS,
      );
    }),
  );
});
