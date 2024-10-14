import { WHISPERIT_BLOCK_STATE_HASH } from "../constants";

suite(["@tier2"], "Block spotlight stats", () => {
  it("are rendered", () => {
    cy.visit(`/blocks/${WHISPERIT_BLOCK_STATE_HASH}/analytics`);
    cy.assertAnalyticsSimplValueEquals("Total User Amounts Transferred", 215.5);
    cy.assertAnalyticsSimplValueEquals(
      "Total Internal Fees Transferred",
      0.012,
    );
    cy.assertAnalyticsSimplValueEquals("Total SNARK Fees", "0.0");
  });
});
