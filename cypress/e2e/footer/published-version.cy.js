suite(["@tier2"], "header", () => {
  it(`links to deployed version on github.com`, () => {
    cy.visit("/");
    cy.intercept("GET", "/summary").as("summaryData");
    cy.get("footer")
      .find(
        'a[href^="https://github.com/Granola-Team/mina-block-explorer/commit/"]',
      )
      .should("exist")
      .invoke("attr", "href")
      .should(
        "match",
        /https:\/\/github\.com\/Granola-Team\/mina-block-explorer\/commit\/[a-f0-9]{8}$/,
      );

    cy.wait("@summaryData").then(() => {
      cy.wait(100);
      cy.get("footer")
        .find('a[href^="https://github.com/Granola-Team/mina-indexer/commit/"]')
        .invoke("attr", "href")
        .should(
          "match",
          /https:\/\/github\.com\/Granola-Team\/mina-indexer\/commit\/[a-f0-9]{7,40}$/,
        );
    });
  });
});
