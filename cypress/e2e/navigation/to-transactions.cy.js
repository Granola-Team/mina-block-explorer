suite(["@CI"],'transaction spotlight', () => {
    let pages = [
        { origin: '/transactions', column: 'Hash', tableHeader: 'Transactions'},
        { origin: '/blocks/3NKnLbpRcFaY9WSzLFa4wYxejhnWBQNEPA2cnesvS75wcuNCr8nA/user-commands', column: 'Hash', tableHeader: 'User Commands'},
        { origin: '/addresses/accounts/B62qiW9Qwv9UnKfNKdBm6hRLNDobv46rVhX1trGdB35YCNT33CSCVt5', column: 'State Hash', tableHeader: 'Transactions'},
    ];

    pages.forEach(({ origin, column, tableHeader }) => it(`is navigated to from ${origin} by clicking link in '${column}'`,() => {
        cy.visit(origin);
        cy.clickLinkInTable(1, column, tableHeader);
        cy.url().should('include', '/transactions/')
    }));
});