import { GENESIS_BLOCK_BLOCK_HASH } from "../constants";

suite(["@tier2"], "block analytic tab", () => {
  // TODO: too expensive to run this query for last 1000 blocks. Re-enable when not
  // it("contains the correct elements", () => {
  //   cy.visit(`/blocks/${GENESIS_BLOCK_BLOCK_HASH}/analytics`);
  //   cy.get(".analytics-sm").should("have.lengthOf", 4);
  //   cy.get(".analytics-lg").should("have.lengthOf", 2);
  // });
});
