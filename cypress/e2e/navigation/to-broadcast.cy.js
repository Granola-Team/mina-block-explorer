suite(["@CI"], "broadcast page", () => {
  let tabs = [
    { text: "Transaction", heading: "Broadcast Signed Transaction" },
    { text: "Delegation", heading: "Broadcast Signed Delegation" },
    { text: "Ledger", heading: "Broadcast Signed Transaction From Ledger" },
  ];

  it(`contains a tab menu with ${tabs.length} tabs`, () => {
    cy.visit("/broadcast/transaction");
    cy.get("menu#tabs li a").should("have.lengthOf", tabs.length);
    tabs.forEach((tab) => {
      cy.get("menu#tabs li a").contains(tab.text).click();
      cy.wait(500);
      cy.get("section h1").contains(tab.heading).should("exist");
      cy.get("section form textarea").should("exist");
      cy.get("section form input[type=submit]").should("exist");
    });
  });
});
