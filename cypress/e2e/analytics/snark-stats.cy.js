suite(["@tier2"], "SNARK stats", () => {
  it("are rendered", () => {
    cy.visit("/analytics/snarks?q-blockheight-gte=0&q-blockheight-lte=10000");
    cy.getBySel("analytics-simple-info").each(($el) =>
      cy.wrap($el).checkNumeric(),
    );
  });
});
