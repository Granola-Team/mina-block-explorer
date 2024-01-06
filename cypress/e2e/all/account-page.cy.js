describe('account page', () => {
    ["Transactions", "SNARK Jobs", "Block Production"].forEach(section => {
        it(`has a ${section} section`, () => {
            cy.visit('/accounts/B62qrCz3ehCqi8Pn8y3vWC9zYEB9RKsidauv15DeZxhzkxL3bKeba5h')
            cy.get('section h1').contains(section,{timeout:60000})
        });
    });

    [{
        dest:"transactions",
        href:"/transactions?account=B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6"
    }, {
        dest:"snarks",
        href:"/snarks?account=B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6"
    }, {
        dest:"blocks",
        href:"/blocks?account=B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6"
    }].forEach(link => {
        it(`has links to ${link.dest} page`,() => {
            cy.visit('/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6');
            let el = cy.get(`a[href="${link.href}"]`, {timeout: 60000})
            el.matchImageSnapshot(`${link.dest}-link`);
        })
    })
})