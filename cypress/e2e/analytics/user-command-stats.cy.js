suite(["@tier2"], "user commands stats", () => {
  it("are rendered", () => {
    cy.visit("/analytics/commands/user?limit=10000");
    cy.get("#total-transferred").should("have.text", "4.861 million MINA", {
      timeout: 15000,
    });
    cy.get("#total-fees").should("have.text", "377.994 MINA");
    cy.get("#total-number-of-transactions").should("have.text", "27,858");
    cy.get("#total-failed-account-creations").should("have.text", "95");
  });
});
