suite(["@CI"], "staking ledger", () => {
  it("shows slot progress message", () => {
    cy.visit("/staking-ledgers");
    cy.get(".additional-info").as("slot-info");

    cy.get("@slot-info")
      .invoke("text")
      .then((epochProgressText) => {
        const info = extractEpochProgress(epochProgressText);
        expect(parseFloat(info.percent)).to.equal(
          parseFloat(((info.slot / info.totalSlots) * 100).toFixed(2)),
        );
      });
  });

  function extractEpochProgress(input) {
    let regex = /(\d+).(\d+)% complete \((\d+)\/(\d+) slots filled\)/;
    const match = input.match(regex);
    return match
      ? {
          percent: match[1] + "." + match[2],
          slot: match[3],
          totalSlots: match[4],
        }
      : null;
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
    cy.get("section").contains("Current Staking Ledger", { timeout: 10000 });
  });

  it("displays link to next stakes page", () => {
    cy.visit("/staking-ledgers");
    cy.get("section").contains("Current Staking Ledger");
    cy.get("a").contains("Next", { timeout: 10000 }).click();
    cy.wait(500);
    cy.get("section").contains("Next Staking Ledger", { timeout: 10000 });
  });

  it("provides navigation between current and Next staking ledger", () => {
    cy.visit("/next-stakes");
    cy.get("section").contains("Next Staking Ledger", { timeout: 10000 });
    cy.get("a").contains("Previous", { timeout: 10000 }).click();
    cy.wait(500);
    cy.get("section").contains("Current Staking Ledger", { timeout: 10000 });
    cy.get("a").contains("Next", { timeout: 10000 }).click();
    cy.wait(500);
    cy.get("section").contains("Next Staking Ledger", { timeout: 10000 });
  });

  it("contains buttons for epoch navigation", () => {
    cy.visit("/staking-ledgers?epoch=67");
    cy.get("section").contains("Epoch 67 Staking Ledger", { timeout: 10000 });
    cy.get("button").contains("Next", { timeout: 10000 }).click();
    cy.wait(500);
    cy.get("section").contains("Epoch 68 Staking Ledger", { timeout: 10000 });
    cy.get("button").contains("Previous", { timeout: 10000 }).click();
    cy.wait(500);
    cy.get("section").contains("Epoch 67 Staking Ledger", { timeout: 10000 });
  });
});
