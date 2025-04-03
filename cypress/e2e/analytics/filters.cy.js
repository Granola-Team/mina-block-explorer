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
suite(["@tier2"], "block height filter url params", () => {
  let cases = [
    {
      url: "/analytics/blocks?q-blockheight-gte=7000",
      expected_gte_input: 7000,
      expected_lte_input: null,
    },
    {
      url: "/analytics/blocks?q-blockheight-lte=9050",
      expected_gte_input: null,
      expected_lte_input: 9050,
    },
    {
      url: "/analytics/blocks?q-blockheight-gte=8050&q-blockheight-lte=9050",
      expected_gte_input: 8050,
      expected_lte_input: 9050,
    },
  ];
  cases.forEach(({ url, expected_gte_input, expected_lte_input }) =>
    it(`work for ${url}`, () => {
      cy.visit(url);
      cy.get("label")
        .contains("Start Block Height")
        .next()
        .as("start-block-height");
      cy.get("label")
        .contains("End Block Height")
        .next()
        .as("end-block-height");
      if (expected_gte_input == null) {
        cy.get("@start-block-height").should("not.have.value");
      } else {
        cy.get("@start-block-height").should("have.value", expected_gte_input);
      }
      if (expected_lte_input == null) {
        cy.get("@end-block-height").should("not.have.value");
      } else {
        cy.get("@end-block-height").should("have.value", expected_lte_input);
      }
    }),
  );
});
