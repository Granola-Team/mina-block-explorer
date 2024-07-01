suite(["@tier1"], "footer content", () => {
  let footer_links = [
    { text: "Granola", selector: 'a[href="https://granola.team"]' },
    {
      text: "Disclaimer",
      selector:
        'a[href="https://gist.github.com/robinbb/05ba138b080ff5a95dcf8bb2d6ae76c5"]',
    },
    {
      text: "Terms",
      selector:
        'a[href="https://gist.github.com/robinbb/15b67f5d39dd47d37ddb88e3201dc311"]',
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
