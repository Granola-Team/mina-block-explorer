describe('navigation to block page', () => {
    [{
        origin: '/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6',
        dest:"transactions",
        href:"/transactions?account=B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6"
    }].forEach(({origin, dest, href}) => it(`has links to ${dest} page`,() => {
        cy.visit(origin);
        cy.get('a').contains("See all transactions", {timeout: 60000}).click();
        cy.url().should('contain', href);
    }));
});