suite(["@CI"],'transaction page', () => {
    [{
        origin: '/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6',
        dest:"transactions",
        href:"/transactions?account=B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6"
    }].forEach(({origin, dest, href}) => it(`is navigated to from ${dest}`,() => {
        cy.visit(origin);
        cy.get('a').contains("See all transactions", {timeout: 60000}).click();
        cy.url().should('contain', href);
    }));

});

suite(["@CI"],'transaction spotlight', () => {
    let pages = [
        { origin: '/transactions', column: 'Hash', tableHeader: 'Transactions'},
        { origin: '/blocks/3NKnLbpRcFaY9WSzLFa4wYxejhnWBQNEPA2cnesvS75wcuNCr8nA', column: 'Hash', tableHeader: 'User Commands'},
    ];

    pages.forEach(({ origin, column, tableHeader }) => it(`is navigated to from ${origin} by clicking link in '${column}'`,() => {
        cy.visit(origin);
        cy.clickLinkInTable(1, column, tableHeader);
        cy.url().should('include', '/transactions/')
    }));
});