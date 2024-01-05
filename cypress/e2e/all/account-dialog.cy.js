describe('account dialog', () => {
    ["Transactions", "SNARK Jobs", "Block Production"].forEach(section => {
        it(`has a ${section} section`, () => {
            cy.visit('summary/accounts/B62qrCz3ehCqi8Pn8y3vWC9zYEB9RKsidauv15DeZxhzkxL3bKeba5h')
            cy.get('section h2').contains(section,{timeout:60000})
        });
    })
    
})