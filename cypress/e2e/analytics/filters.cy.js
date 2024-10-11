suite(["@tier2"], "block height filters", () => {
  let pages = [
    {
      url: "/analytics/blocks",
    },
    {
      url: "/analytics/commands/user",
    },
    {
      url: "/analytics/snarks",
    },
  ];
  pages.forEach(({ url }) =>
    it(`work on ${url}`, () => {
      cy.visit(url);
      cy.get("label")
        .contains("Start Block Height")
        .next()
        .as("start-block-height");
      cy.get("label")
        .contains("End Block Height")
        .next()
        .as("end-block-height");
      cy.get("button").contains("Apply").as("submit");

      cy.get("@submit").click();
      cy.get("#input-validation").should(
        "have.text",
        "Missing start block height",
      );

      cy.get("@start-block-height").clear().type(9000);
      cy.get("#input-validation").should("not.exist");
      cy.get("@submit").click();
      cy.get("#input-validation").should(
        "have.text",
        "Missing end block height",
      );

      cy.get("@end-block-height").clear().type(9000);
      cy.get("#input-validation").should("not.exist");
      cy.get("@submit").click();
      cy.get("#input-validation").should(
        "have.text",
        "End block must be larger than start block",
      );

      cy.get("@end-block-height").clear().type(9001);
      cy.get("#input-validation").should("not.exist");
      cy.get("@submit").click();
      cy.get("#input-validation").should("not.exist");

      cy.get("@start-block-height").clear().type(6000);
      cy.get("#input-validation").should("not.exist");
      cy.get("@submit").click();
      cy.get("#input-validation").should(
        "have.text",
        "Block range must not exceed 2000",
      );
    }),
  );
});
