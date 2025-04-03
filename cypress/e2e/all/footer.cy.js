let devices = ["iphone-xr", "macbook-11"];
let pages = ["/blocks", "/commands", "/snarks", "/staking-ledgers"];
suite(["@tier2"], "desktop footer", () => {
  devices.forEach((device) => {
    pages.forEach((page) => {
      it(`exist on ${page} page on device ${device}`, () => {
        cy.viewport(device);
        cy.visit(page);
        cy.get("footer").should("exist");
      });
    });
  });
});
