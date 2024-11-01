suite(["@tier2"], "Staker Leaderboard", () => {
  it("defaults to latest epoch", () => {
    cy.visit("/analytics/");
    cy.get("menu#tabs").find("li").contains("Staker Leaderboard").click();
    cy.url().should("include", "epoch=1");
    cy.contains("label", "Epoch").parent().find("input").as("epoch-input");
    cy.get("@epoch-input").should("have.value", 1);
  });
});
