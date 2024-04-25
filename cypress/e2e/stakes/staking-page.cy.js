suite(["@CI"], "staking ledger", () => {
  it("shows slot progress indicator", () => {
    cy.visit("/staking-ledgers");
    cy.get(".pg-container").as("progress");
    cy.get(".pg-container .pg-completeness").as("pg-completeness");
    cy.get(".pg-container .pg-slot").as("pg-slot");
    cy.get(".pg-container .pg-total-slots").as("pg-total-slots");

    cy.get("@pg-total-slots")
      .invoke("text")
      .then((totalSlots) => {
        expect(extractTotalSlots(totalSlots)).to.equal("7140");
        let totalSlotsInt = parseInt(extractTotalSlots(totalSlots));
        cy.log("Total slots: ", totalSlotsInt);
        cy.get("@pg-slot")
          .invoke("text")
          .then((slot) => {
            let slotInt = parseInt(extractSlot(slot));
            cy.log("Current Slot: ", slotInt);
            cy.get("@pg-completeness")
              .invoke("text")
              .then((progress) => {
                let progressFloat = parseFloat(
                  extractProgressPercent(progress),
                );
                cy.log("Percent Complete: ", progressFloat);
                expect(progressFloat).to.equal(
                  parseFloat(((slotInt / totalSlotsInt) * 100).toFixed(2)),
                );
              });
          });
      });
  });

  function extractProgressPercent(input) {
    let regex = /Epoch is (\d+).(\d+)% complete/;
    const match = input.match(regex);
    return match ? match[1] + "." + match[2] : null;
  }

  function extractSlot(input) {
    let regex = /Current slot: (\d+)/;
    const match = input.match(regex);
    return match ? match[1] : null;
  }

  function extractTotalSlots(input) {
    let regex = /Epoch slots: (\d+)/;
    const match = input.match(regex);
    return match ? match[1] : null;
  }

  it("only has large positive stakes", () => {
    cy.visit("/staking-ledgers");
    cy.aliasTableColumnValue("Current Staking Ledger", "Stake", "stake-value");
    cy.get("@stake-value")
      .invoke("text")
      .then((text) => {
        cy.log(text);
        var numText = text.replace("mina", "").trim();
        var num = parseFloat(numText);
        expect(num).to.be.gt(0);
      });
  });

  it("defaults to current epoch", () => {
    cy.visit("/staking-ledgers");
    cy.get("section").contains("Current Staking Ledger");
  });

  it("displays link to next stakes page", () => {
    cy.visit("/staking-ledgers");
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
    cy.visit("/staking-ledgers?epoch=67");
    cy.get("section").contains("Epoch 67 Staking Ledger");
    cy.get("button").contains("Next").click();
    cy.wait(500);
    cy.get("section").contains("Epoch 68 Staking Ledger");
    cy.get("button").contains("Previous").click();
    cy.wait(500);
    cy.get("section").contains("Epoch 67 Staking Ledger");
  });
});
