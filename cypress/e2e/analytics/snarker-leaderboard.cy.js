describe("SNARKer Leaderboard", () => {
  it("defaults to latest epoch", () => {
    cy.visit("/analytics/");
    cy.get("menu#tabs").find("li").contains("Snarker Leaderboard").click();
    cy.url().should("include", "epoch=0");
    cy.contains("label", "Epoch").parent().find("input").as("epoch-input");
    cy.get("@epoch-input").should("have.value", 0);
  });
});
