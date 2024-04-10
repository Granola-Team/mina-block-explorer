suite(["@CI"], "staking ledger", () => {
  it("only has large positive stakes", () => {
    cy.visit("/stakes");
    cy.aliasTableColumnValue("Current Staking Ledger", "Stake", "stake-value");
    cy.get("@stake-value")
      .invoke("text")
      .then((text) => {
        cy.log(text);
        var numText = text.replace("mina", "").trim();
        var num = Number(numText);
        expect(num).to.be.gt(0);
      });
  });

  it("defaults to current epoch", () => {
    cy.visit("/stakes");
    cy.get("section").contains("Current Staking Ledger");
  });

  it("displays link to next stakes page", () => {
    cy.visit("/stakes");
    cy.get("section").contains("Current Staking Ledger");
    cy.get("a").contains("Next Stakes").click();
    cy.wait(500);
    cy.get("section").contains("Next Staking Ledger");
  });

  it("provides navigation between current and Next staking ledger", () => {
    cy.visit("/next-stakes");
    cy.get("section").contains("Next Staking Ledger");
    cy.get("a").contains("Current Stakes").click();
    cy.wait(500);
    cy.get("section").contains("Current Staking Ledger");
    cy.get("a").contains("Next Stakes").click();
    cy.wait(500);
    cy.get("section").contains("Next Staking Ledger");
  });

  it("contains buttons for epoch navigation", () => {
    cy.visit("/stakes?epoch=67");
    cy.get("section").contains("Epoch 67 Staking Ledger");
    cy.get("button").contains("Next").click();
    cy.wait(500);
    cy.get("section").contains("Epoch 68 Staking Ledger");
    cy.get("button").contains("Previous").click();
    cy.wait(500);
    cy.get("section").contains("Epoch 67 Staking Ledger");
  });
});
