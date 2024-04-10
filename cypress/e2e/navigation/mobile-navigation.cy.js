const pages = require("../../pages.json");

suite(["@CI"], "mobile menu", () => {
  beforeEach(() => {
    cy.viewport("iphone-xr");
    cy.visit("/");
    cy.openMobileMenu();
  });

  pages.forEach((url) =>
    it(`provides navigation to ${url}`, () => {
      cy.get(`nav a[href^="${url}"]`).first().click();
      cy.url().should("contain", url);
    }),
  );
});
