suite(["@CI"], "tab count and row count", () => {
  let tabs = ["SNARK Jobs", "User Commands", "Internal Commands"];

  tabs.forEach((tab) =>
    it(`match on ${tab} tab (random block audit)`, () => {
      cy.visit("/blocks");
      cy.clickLinkInTable(1, "State Hash", "Blocks");

      cy.url().should("include", "/spotlight");
      cy.contains("a.tab", tab).as("tab");
      cy.get("@tab").find(".number-bubble").as("tab-count");

      cy.get("@tab").click();
      cy.get("@tab-count")
        .invoke("text")
        .then((count) => {
          let count = Number(count);
          if (count > 0) {
            cy.aliasTableRows(tab, "tr");
            cy.get("@tr").should("have.lenghtOf", count);
          } else {
            cy.get("table").should("not.exist");
          }
        });
    }),
  );
});
