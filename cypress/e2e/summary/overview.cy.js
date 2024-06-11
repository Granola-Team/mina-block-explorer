suite(["@CI"], "blockchain overview", () => {
  it("displays non-zero metrics", () => {
    cy.visit("/blocks");
    cy.wait(1000);

    let summaryItems = [
      {
        id: "#uniqueBlockProducers",
        label: "Unique Producers of last 1000 Blocks",
      },
    ];
    summaryItems.forEach(({ id, label }) => {
      cy.get(`${id} .loading-placeholder`).should("not.exist");
      cy.get(id)
        .invoke("text")
        .then((text) => {
          let value = parseInt(text);
          expect(value).to.be.gt(0);
        });
      cy.get(id).siblings("label").should("have.text", label);
    });
  });
});