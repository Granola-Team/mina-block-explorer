describe('navigation to account spotlight', () => {
    let dialogs = [{
        origin: '/summary/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6',
        selector: '#viewmore a'
    },
    {
        origin: '/blocks/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6',
        selector: '#viewmore a'
    }];

    dialogs.forEach(({ origin, selector }) => it(`navigates to the account spotlight page from page ${origin}`,() => {
        cy.visit(origin);
        let link = cy.get(selector, {timeout: 20000}).first();
        link.then($a => {
            link.click({force:true});
            cy.url().should('include',$a.attr('href'), {timeout: 10000})
        })
    }));


});