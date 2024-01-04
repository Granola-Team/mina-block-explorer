describe('account dialog', () => {
    it('has a transaction section', () => {
        cy.visit('summary/accounts/B62qrCz3ehCqi8Pn8y3vWC9zYEB9RKsidauv15DeZxhzkxL3bKeba5h')
        cy.get('section h2').contains('Transactions',{timeout:10000})
        cy.get('section h2').contains('SNARK Jobs',{timeout:20000})
    })
})