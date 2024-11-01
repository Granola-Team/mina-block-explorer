suite(["@tier2"], "SNARKer Leaderboard", () => {
  it("defaults to latest epoch", () => {
    cy.visit("/analytics");
    cy.wait(100);
    cy.get("menu#tabs").find("li").contains("Snarker Leaderboard").click();
    cy.url().should("include", "epoch=1");
    cy.contains("label", "Epoch").parent().find("input").as("epoch-input");
    cy.get("@epoch-input").should("have.value", 1);
  });
});
