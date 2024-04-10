suite(["@CI"], "block analytic tab", () => {
  it("contains the correct elements", () => {
    cy.visit(
      "/blocks/3NKdghxnw7vQmVmj1G3MK1PQXYU5dDH1BQV3cCjXjViPW47L6hHJ/analytics",
    );

    cy.get(".analytics-sm").should("have.lengthOf", 4);
    cy.get(".analytics-lg").should("have.lengthOf", 2);
  });
});
