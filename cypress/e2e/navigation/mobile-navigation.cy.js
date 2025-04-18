let pages = ["/blocks", "/commands", "/snarks", "/staking-ledgers"];
describe("mobile menu", () => {
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
  it("has all menu items visible", () => {
    cy.get("a.nav-link span").each(($el) => {
      cy.wrap($el).should("be.visible");
    });
  });
});
