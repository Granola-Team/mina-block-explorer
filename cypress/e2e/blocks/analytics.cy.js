import { GENESIS_BLOCK_BLOCK_HASH } from "../constants";

suite(["@tier1"], "block analytic tab", () => {
  it("contains the correct elements", () => {
    cy.visit(`/blocks/${GENESIS_BLOCK_BLOCK_HASH}/analytics`);

    cy.get(".analytics-sm").should("have.lengthOf", 4);
    cy.get(".analytics-lg").should("have.lengthOf", 2);
  });
});
