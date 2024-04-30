suite(["@CI"], "tab count and pagination count", () => {
  let tabs = ["SNARK Jobs", "User Commands", "Internal Commands"];

  tabs.forEach((tab) =>
    it(`match on ${tab} tab (random block audit)`, () => {
      cy.visit("/blocks");
      cy.clickLinkInTable(1, "State Hash", "Blocks");

      cy.url().should("include", "/spotlight");
      cy.get("a").contains(tab).as("tab");
      cy.get("@tab").siblings(".number-bubble").first().as("tab-count");

      cy.get("@tab").click();
      cy.get("@tab-count")
        .invoke("text")
        .then((count) => {
          if (Number(count) > 0) {
            cy.scrollTo("bottom", { ensureScrollable: false });
            cy.get(".pagination-controls").children().first().contains(count);
          } else {
            cy.get("table").should("not.exist");
          }
        });
    }),
  );
});
