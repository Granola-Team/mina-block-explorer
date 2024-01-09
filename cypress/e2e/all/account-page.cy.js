describe('account page', () => {
    it(`has all sections`, () => {
        cy.visit('/accounts/B62qrCz3ehCqi8Pn8y3vWC9zYEB9RKsidauv15DeZxhzkxL3bKeba5h');
        ["Transactions", "SNARK Jobs", "Block Production"].forEach(section => {
            cy.get('section h1').contains(section,{timeout:60000})
        });
    });
    
})