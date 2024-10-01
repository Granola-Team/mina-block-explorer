suite(["@tier2"], "user commands stats", () => {
  it("are rendered", () => {
    cy.visit("/analytics/commands/user?limit=10000");
    cy.get("#total-transferred").should("have.text", "4.859", {
      timeout: 15000,
    });
    cy.get("#total-fees").should("have.text", "377.979");
    cy.get("#total-number-of-transactions").should("have.text", "27,852");
    cy.get("#total-failed-account-creations").should("have.text", "95");
  });
});
