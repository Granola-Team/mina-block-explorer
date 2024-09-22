suite(["@tier2"], "user commands stats", () => {
  it("are rendered", () => {
    cy.visit("/analytics/user/commands?limit=10000");
    cy.get("#total-transferred").should("have.value", "4.861 million MINA");
    cy.get("#total-fees").should("have.value", "377.994 MINA");
    cy.get("#total-number-of-transactions").should("have.value", "27,858");
    cy.get("#total-failed-account-creations").should("have.value", "95");
  });
});
