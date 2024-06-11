suite(["@tier1"], "footer content", () => {
  let footer_links = [
    { text: "Granola", selector: 'a[href="https://granola.team"]' },
    {
      text: "Docs",
      selector:
        'a[href="https://github.com/Granola-Team/mina-block-explorer/blob/main/DOCS/SITE_DOCS.md"]',
    },
    {
      text: "Terms",
      selector:
        'a[href="https://gist.github.com/jhult/0a633e7771a695b0ffa529ab55722523"]',
    },
    {
      text: "Support",
      selector: 'a[href="https://docs.minaexplorer.com/minaexplorer/get-help"]',
    },
  ];

  it("is present and visible on mobile", () => {
    cy.visit("/");
    cy.viewport("iphone-xr");
    cy.scrollTo("bottom");

    footer_links.forEach(({ text, selector }) => {
      cy.get("footer").contains(selector, text).should("be.visible");
    });
  });

  it("is present and visible on desktop", () => {
    cy.visit("/");
    cy.scrollTo("bottom");

    footer_links.forEach(({ text, selector }) => {
      cy.get("footer").contains(selector, text).should("be.visible");
    });
  });
});
