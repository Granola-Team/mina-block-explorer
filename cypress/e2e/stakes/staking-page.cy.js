describe('staking ledger', () => {

    it('defaults to current epoch',() => {
        cy.visit("/stakes");
        cy.get('section').contains("Current Staking Ledger");
    });

    it('displays link to next stakes page',() => {
        cy.visit("/stakes");
        cy.get('section').contains("Current Staking Ledger");
        cy.get('a').contains('Next Stakes').click();
        cy.wait(500);
        cy.get('section').contains("Next Epoch Staking Ledger");
    });

    it('provides navigation between current and next epoch staking ledger',() => {
        cy.visit("/next-stakes");
        cy.get('section').contains("Next Epoch Staking Ledger");
        cy.get('a').contains('Current Stakes').click();
        cy.wait(500);
        cy.get('section').contains("Current Staking Ledger");
        cy.get('a').contains('Next Stakes').click();
        cy.wait(500);
        cy.get('section').contains("Next Epoch Staking Ledger");
        
    });

    it('contains buttons for epoch navigation',() => {
        cy.visit("/stakes?epoch=67");
        cy.get('section').contains("Epoch 67 Staking Ledger");
        cy.get('button').contains("Next").click();
        cy.wait(500);
        cy.get('section').contains("Epoch 68 Staking Ledger");
        cy.get('button').contains("Previous").click();
        cy.wait(500);
        cy.get('section').contains("Epoch 67 Staking Ledger");
    })
})