suite(["tier1"], "header", () => {
  it(`links to deployed version on github.com`, () => {
    cy.visit("/");
    cy.get("footer")
      .find(
        'a[href^="https://github.com/Granola-Team/mina-block-explorer/commit/"]',
      )
      .should("exist");
  });
});
