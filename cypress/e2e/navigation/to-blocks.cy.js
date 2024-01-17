describe('block page', () => {
    [{
        origin: '/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6',
        dest:"blocks",
        href:"/blocks?account=B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6"
    }].forEach(({origin, dest, href}) => it(`is navigated to from ${dest} page`,() => {
        cy.visit(origin);
        cy.get('a').contains("See all block production", {timeout: 60000}).click();
        cy.url().should('contain', href);
    }));
});