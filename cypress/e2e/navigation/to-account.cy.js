import { DEFAULT_ACCOUNT_PK } from "../constants";

suite(["@CI"],'account page', () => {
    let pages = [
        { origin: '/summary/accounts/B62qqW8uKTxHZueKJwsoPY8NZcKVeDK4bLEHRkpMM2uKtEmmqLbkiQC', column: 'Counterparty', tableHeader: 'Transactions', tableHeaderEl: 'h2', transposed: true},
        { origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`, column: 'Counterparty', tableHeader: 'Transactions' },
        { origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`, column: 'Block Producer', tableHeader: 'Block Production' },
        { origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`, column: 'Coinbase Receiver', tableHeader: 'Block Production' },
        { origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`, column: 'Prover', tableHeader: 'SNARK Jobs' },
        { origin: '/snarks', column: 'Prover', tableHeader: 'SNARKs' },
        { origin: '/stakes', column: 'Key', tableHeader: 'Current Staking Ledger' },
        { origin: '/stakes', column: 'Delegate', tableHeader: 'Current Staking Ledger' },
        { origin: '/next-stakes', column: 'Key', tableHeader: 'Next Staking Ledger' },
        { origin: '/next-stakes', column: 'Delegate', tableHeader: 'Next Staking Ledger' },
        { origin: '/transactions', column: 'From', tableHeader: 'Transactions'},
        { origin: '/transactions', column: 'To', tableHeader: 'Transactions'},
        { origin: '/blocks/3NKnLbpRcFaY9WSzLFa4wYxejhnWBQNEPA2cnesvS75wcuNCr8nA/user-commands', column: 'From', tableHeader: 'User Commands'},
        { origin: '/blocks/3NKnLbpRcFaY9WSzLFa4wYxejhnWBQNEPA2cnesvS75wcuNCr8nA/user-commands', column: 'To', tableHeader: 'User Commands'},
        { origin: '/blocks/3NKjn8eQiAdwHMeenVuHKmqTVarJzPU7bfPnvSu74XuXTdzhXpj4/snark-jobs', column: 'Prover', tableHeader: 'SNARK Jobs'},
        { origin: '/blocks/3NLXaJBYriRYe8LQUNwgSFsUvuikjkL8SDo1MHKRYsfRA4FjCsEv/internal-commands', column: 'Recipient', tableHeader: 'Internal Commands'},
    ];

    pages.forEach(({ origin, column, tableHeader, tableHeaderEl='h1', transposed }) => it(`is navigated to from ${origin} by clicking link in '${column}'`,() => {
        cy.visit(origin);
        if (transposed) {
            cy.clickLinkInTransposedTable(column, tableHeader, tableHeaderEl);
        } else {
            cy.clickLinkInTable(1, column, tableHeader, tableHeaderEl);
        }
        
        cy.url().should('include', '/accounts/')
    }));
})