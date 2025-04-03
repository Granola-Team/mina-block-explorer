suite(["@tier2"], "user commands stats", () => {
  it("are rendered", () => {
    cy.visit(
      "/analytics/commands/user?q-blockheight-gte=0&q-blockheight-lte=10000",
    );
    cy.getBySel("analytics-simple-info").each(($el) =>
      cy.wrap($el).checkNumeric(),
    );
  });
});
