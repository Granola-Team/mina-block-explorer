import { APPLIED_TXN_BLOCK_STATE_HASH } from "../constants";
suite(["@tier2"], "Block spotlight stats", () => {
  it("are rendered", () => {
    cy.visit(`/blocks/${APPLIED_TXN_BLOCK_STATE_HASH}/analytics`);
    cy.getBySel("analytics-simple-info").each(($el) =>
      cy.wrap($el).checkNumeric(),
    );
  });
});
