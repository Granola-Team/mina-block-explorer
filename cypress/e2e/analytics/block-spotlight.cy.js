import { WHISPERIT_BLOCK_STATE_HASH } from "../constants";

suite(["@tier2"], "Block spotlight stats", () => {
  it("are rendered", () => {
    cy.visit(`/blocks/${WHISPERIT_BLOCK_STATE_HASH}/analytics`);
    cy.getBySel("analytics-simple-info").each(($el) =>
      cy.wrap($el).checkNumeric(),
    );
  });
});
