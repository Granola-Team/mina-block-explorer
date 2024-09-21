suite(["@tier2"], "fitler", () => {
  let pages = ["/analytics/commands/user", "/analytics/snarks"];
  pages.forEach((page) =>
    it(`has defaults for ${page}`, () => {
      cy.visit(page);
      cy.get("#block-limit").should("have.value", 1000);
      cy.url().should("include", "limit=1000");
    }),
  );
});
