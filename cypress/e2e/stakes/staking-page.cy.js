suite(["@CI"], "staking ledger", () => {
  // TODO: enable when all epochs available
  // it("shows slot progress message", () => {
  //   cy.visit("/staking-ledgers?epoch=0");
  //   cy.get(".staking-ledger-percent-complete").as("slot-info");

  //   cy.get("@slot-info")
  //     .invoke("text")
  //     .then((epochProgressText) => {
  //       const info = extractEpochProgress(epochProgressText);
  //       expect(parseFloat(info.percent)).to.equal(
  //         parseFloat(((info.slot / info.totalSlots) * 100).toFixed(2)),
  //       );
  //     });
  // });

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
    cy.visit("/staking-ledgers?epoch=0");
    cy.get(".loading-placeholder").should("exist");
    cy.get(".loading-placeholder").should("not.exist");
    cy.aliasTableColumnValue("Staking Ledger", "Stake", "stake-value");
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
    cy.visit("/staking-ledgers?epoch=0");
    cy.get("section").contains("Staking Ledger");
  });

  // TODO: enable when all epochs available
  // it("displays link to next stakes page", () => {
  //   cy.visit("/staking-ledgers?epoch=0");
  //   cy.get("section").contains("Staking Ledger");
  //   cy.get("section").contains("a", "Next").click();
  //   cy.wait(500);
  //   cy.get("section").contains("Next Staking Ledger");
  // });

  // TODO: enable when page staking ledger working again
  // it("contains buttons for epoch navigation", () => {
  //   cy.visit("/staking-ledgers?epoch=67");
  //   cy.get("section").contains("Staking Ledger - Epoch 67");
  //   cy.get("section").contains("button", "Next").click();
  //   cy.wait(500);
  //   cy.get("section").contains("Staking Ledger - Epoch 68");
  //   cy.get("button").contains("Previous").click();
  //   cy.wait(500);
  //   cy.get("section").contains("Staking Ledger - Epoch 67");
  // });
});
