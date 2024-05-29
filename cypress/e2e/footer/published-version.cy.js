suite(["@CI"], "header", () => {
  it(`links to deployed version on github.com`, () => {
    cy.visit("/");
    cy.get("header")
      .find(
        'a[href^="https://github.com/Granola-Team/mina-block-explorer/commit/"]',
      )
      .should("exist");
  });
});
