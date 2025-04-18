describe("footer content", () => {
  let footer_links = [
    { text: "Granola", selector: 'a[href="https://granola.team"]' },
    {
      text: "Docs",
      selector: 'a[href="https://docs.minasearch.com"]',
    },
    {
      text: "API",
      selector: 'a[href="https://docs.minasearch.com/apis"]',
    },
    {
      text: "Discord",
      selector: 'a[href="https://discord.gg/Zvu6XHNCxj"]',
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
