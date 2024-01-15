describe('staking ledger', () => {

    it('defaults to current epoch',() => {
        cy.visit("/stakes");
        cy.get('section').contains("Current Staking Ledger");
    });

    it('contains buttons for epoch navigation',() => {
        cy.visit("/stakes?epoch=67");
        cy.get('section').contains("Epoch 67 Staking Ledger");
        cy.get('button').contains("Next").click();
        cy.get('section').contains("Epoch 68 Staking Ledger");
        cy.get('button').contains("Previous").click();
        cy.get('section').contains("Epoch 67 Staking Ledger");
    })
})