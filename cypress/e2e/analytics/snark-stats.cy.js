suite(["@tier2"], "SNARK stats", () => {
  it("are rendered", () => {
    cy.visit("/analytics/snarks?q-blockheight-gte=0&q-blockheight-lte=10000");
    cy.get("#fee-free-work").should("have.text", "46,390", {
      timeout: 15000,
    });
    cy.get("#for-fee-jobs").should("have.text", "717");
    cy.get("#total-snark-jobs").should("have.text", "47,107");
    cy.get("#highest-fee").should("have.text", "700");
  });
});
