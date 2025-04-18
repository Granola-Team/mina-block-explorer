describe("back navigation from redirection", () => {
  let navigations = [
    ["/commands/user", "/", "/blocks"],
    [
      "/blocks",
      "/blocks/3NLhfsN1QPHsKzqu6RaVyLKmMCYTXnkrueMoWpmRQErfBkh6v6as",
      "/blocks/3NLhfsN1QPHsKzqu6RaVyLKmMCYTXnkrueMoWpmRQErfBkh6v6as/spotlight",
    ],
    [
      "/blocks",
      "/addresses/accounts/B62qqgGAQfpFhX8G1iF253C37CMsj6ypn77C9fr3Y17iU6B5Ft4XjPo",
      "/addresses/accounts/B62qqgGAQfpFhX8G1iF253C37CMsj6ypn77C9fr3Y17iU6B5Ft4XjPo/commands/user",
    ],
    ["/blocks", "/commands", "/commands/user"],
    ["/blocks", "/broadcast", "/broadcast/transaction"],
    ["/blocks", "/analytics", "/analytics/blocks"],
  ];
  navigations.forEach(([origin, destination, redirection]) => {
    it(`works when navigating from "${origin}" to "${destination}"`, () => {
      cy.visit(origin);
      cy.visit(destination);
      cy.url().should("include", redirection);
      cy.go("back");
      cy.url().should("include", origin);
    });
  });
});
