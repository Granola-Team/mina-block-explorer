suite(["@tier2"], "SNARK stats", () => {
  it("are rendered", () => {
    cy.visit(
      "/analytics/snarks?q-blockheight-gte=359606&q-blockheight-lte=359706",
    );
    cy.getBySel("analytics-simple-info").each(($el) =>
      cy.wrap($el).checkNumeric(),
    );
  });
});
