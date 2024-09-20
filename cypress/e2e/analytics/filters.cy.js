suite(["@tier2"], "fitler", () => {
  it("has defaults for the snark analytics page", () => {
    cy.visit("/analytics/snarks");
    cy.get("#block-limit").should("have.value", 1000);
    cy.url().should("include", "limit=1000");
  });
});
