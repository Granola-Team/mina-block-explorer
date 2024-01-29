suite(["@CI"],'account page', () => {
    let pages = [
        { origin: '/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6', column: 'From', tableHeader: 'Transactions' },
        { origin: '/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6', column: 'To', tableHeader: 'Transactions' },
        { origin: '/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6', column: 'Block Producer', tableHeader: 'Block Production' },
        { origin: '/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6', column: 'Coinbase Receiver', tableHeader: 'Block Production' },
        { origin: '/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6', column: 'Prover', tableHeader: 'SNARK Jobs' },
        { origin: '/snarks', column: 'Prover', tableHeader: 'SNARKs' },
        { origin: '/stakes', column: 'Key', tableHeader: 'Current Staking Ledger' },
        { origin: '/stakes', column: 'Delegate', tableHeader: 'Current Staking Ledger' },
        { origin: '/next-stakes', column: 'Key', tableHeader: 'Next Staking Ledger' },
        { origin: '/next-stakes', column: 'Delegate', tableHeader: 'Next Staking Ledger' },
        { origin: '/transactions', column: 'From', tableHeader: 'Transactions'},
        { origin: '/transactions', column: 'To', tableHeader: 'Transactions'},
        { origin: '/blocks/3NKnLbpRcFaY9WSzLFa4wYxejhnWBQNEPA2cnesvS75wcuNCr8nA', column: 'From', tableHeader: 'User Commands'},
        { origin: '/blocks/3NKnLbpRcFaY9WSzLFa4wYxejhnWBQNEPA2cnesvS75wcuNCr8nA', column: 'To', tableHeader: 'User Commands'},
        { origin: '/blocks/3NKjn8eQiAdwHMeenVuHKmqTVarJzPU7bfPnvSu74XuXTdzhXpj4', column: 'Prover', tableHeader: 'SNARK Jobs'},
        { origin: '/blocks/3NLXaJBYriRYe8LQUNwgSFsUvuikjkL8SDo1MHKRYsfRA4FjCsEv', column: 'Recipient', tableHeader: 'Fee Transfers'},
    ];

    pages.forEach(({ origin, column, tableHeader }) => it(`is navigated to from ${origin} by clicking link in '${column}'`,() => {
        cy.visit(origin);
        cy.clickLinkInTable(1, column, tableHeader);
        cy.url().should('include', '/accounts/')
    }));
})