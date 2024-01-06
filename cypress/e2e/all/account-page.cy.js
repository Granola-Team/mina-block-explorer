describe('account page', () => {
    ["Transactions", "SNARK Jobs", "Block Production"].forEach(section => {
        it(`has a ${section} section`, () => {
            cy.visit('/accounts/B62qrCz3ehCqi8Pn8y3vWC9zYEB9RKsidauv15DeZxhzkxL3bKeba5h')
            cy.get('section h1').contains(section,{timeout:60000})
        });
    });

    ["See all transactions", "See all snark jobs", "See all block production"].forEach(text => {
        it(`has links to ${text}`,() => {
            cy.visit('/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6');
            let link = cy.get('a').contains(text, {timeout: 60000})
            link.parent().matchImageSnapshot(`${text}-link`);
        })
    })
})