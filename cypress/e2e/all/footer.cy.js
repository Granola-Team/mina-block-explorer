const pages = require("../../pages.json");
const devices = require("../../devices.json");

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
