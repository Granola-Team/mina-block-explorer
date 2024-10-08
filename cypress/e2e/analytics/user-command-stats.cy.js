suite(["@tier2"], "user commands stats", () => {
  it("are rendered", () => {
    cy.visit(
      "/analytics/commands/user?q-blockheight-gte=0&q-blockheight-lte=10000",
    );
    cy.get("#total-transferred").should("have.text", "4.859", {
      timeout: 15000,
    });
    cy.get("#total-fees").should("have.text", "378.016");
    cy.get("#total-number-of-transactions").should("have.text", "27,862");
    cy.get("#total-failed-account-creations").should("have.text", "95");
  });
});
