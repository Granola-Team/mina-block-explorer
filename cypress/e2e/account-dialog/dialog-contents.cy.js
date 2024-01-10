describe('dialog',()=>{

    beforeEach(() => {
        cy.visit('/summary/accounts/B62qrCz3ehCqi8Pn8y3vWC9zYEB9RKsidauv15DeZxhzkxL3bKeba5h');
    })
    
    it(`has correct sections`, () => {
        ["Transactions", "SNARK Jobs", "Block Production"].forEach(section => {
            cy.get('section h2').contains(section,{timeout:60000})
        });
    });
    
})