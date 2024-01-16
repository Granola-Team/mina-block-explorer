describe('navigation account page', () => {
    let pages = [
        { origin: '/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6', column: 'From', tableHeader: 'Transactions' },
        { origin: '/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6', column: 'To', tableHeader: 'Transactions' },
        { origin: '/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6', column: 'Block Producer', tableHeader: 'Block Production' },
        { origin: '/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6', column: 'Coinbase Receiver', tableHeader: 'Block Production' },
        { origin: '/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6', column: 'Prover', tableHeader: 'SNARK Jobs' },
        { origin: '/snarks', column: 'Prover', tableHeader: 'SNARKs' },
        { origin: '/stakes', column: 'Key', tableHeader: 'Current Staking Ledger' },
        { origin: '/stakes', column: 'Delegate', tableHeader: 'Current Staking Ledger' },
        { origin: '/transactions', column: 'From', tableHeader: 'Transactions'},
        { origin: '/transactions', column: 'To', tableHeader: 'Transactions'},
    ];

    pages.forEach(({ origin, column, tableHeader }) => it(`navigates to the account ${origin} by clicking link in '${column}'`,() => {
        cy.visit(origin);
        cy.clickLinkInTable(1, column, tableHeader);
        cy.url().should('include', '/accounts/')
    }));
})