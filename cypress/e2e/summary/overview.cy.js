suite(["@CI"], "blockchain overview", () => {
  it("displays unique block producers", () => {
    cy.visit("/blocks");
    cy.wait(1000);
    cy.get("#uniqueBlockProducers .loading-placeholder").should("not.exist");
    cy.get("#uniqueBlockProducers")
      .invoke("text")
      .then((text) => {
        let value = parseInt(text);
        expect(value).to.be.gt(0);
      });
    cy.get("#uniqueBlockProducers")
      .siblings("label")
      .should("have.text", "Block Producers in last 1000 Blocks");
  });
});
